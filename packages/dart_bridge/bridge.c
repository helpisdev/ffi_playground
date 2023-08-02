#include "bridge.h"
#include "include/uthash.h"
#include <stdlib.h>

// Hashmap struct
struct HandleRegistry {
  Dart_PersistentHandle handle;
  char *key;         // Key for this handle
  UT_hash_handle hh; // Makes this structure hashable
};

typedef struct HandleRegistry CBRegistry;

CBRegistry *handles = NULL; // The hashmap

// Function to add a handle to the map
void addCallback(Dart_PersistentHandle handle, const char *key) {
  CBRegistry *cbEntry;

  HASH_FIND_STR(handles, key, cbEntry); /* id already in the hash? */
  if (cbEntry == NULL) {
    cbEntry = (CBRegistry *)malloc(sizeof(CBRegistry));
    cbEntry->key = strdup(key);
    HASH_ADD_STR(handles, key, cbEntry);
  }
  cbEntry->handle = handle;
}

void (*cbInvoker)(Dart_Handle callback, const char *msg, size_t msgLen);

// Initialize `dart_api_dl.h`
DART_EXPORT intptr_t init(void *data) { return Dart_InitializeApiDL(data); }

DART_EXPORT void registerCBInvoker(CallbackInvoker *invoker) {
  cbInvoker = invoker;
}

DART_EXPORT void registerCB(Dart_Handle callback, const char *id) {
  addCallback(Dart_NewPersistentHandle_DL(callback), id);
}

// Add callback id to invoke
DART_EXPORT void invokeCB(const char *msg, size_t msgLen, const char *id) {
  CBRegistry *cbEntry;

  HASH_FIND_STR(handles, id, cbEntry);
  if (cbEntry != NULL) {
    Dart_Handle retrievedCBHandle =
        Dart_HandleFromPersistent_DL(cbEntry->handle);
    cbInvoker(retrievedCBHandle, msg, msgLen);
  }
}

// Release all callbacks together
DART_EXPORT void releaseCB() {
  CBRegistry *handle, *tmp;
  HASH_ITER(hh, handles, handle, tmp) {
    Dart_DeletePersistentHandle_DL(handle->handle);
    HASH_DEL(handles, handle);
    free(handle);
  }
}
