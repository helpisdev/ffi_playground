#include "../../include/bridge.h"
#include "../../include/headers/dart_api_dl.h"
#include "../../include/headers/dart_native_api.h"
#include "../../include/headers/uthash.h"
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
void add_callback(Dart_PersistentHandle handle, const char *key) {
  CBRegistry *cbEntry;

  HASH_FIND_STR(handles, key, cbEntry); /* id already in the hash? */
  if (cbEntry == NULL) {
    cbEntry = (CBRegistry *)malloc(sizeof(CBRegistry));
    cbEntry->key = strdup(key);
    HASH_ADD_STR(handles, key, cbEntry);
  }
  cbEntry->handle = handle;
}

void (*cbInvoker)(Dart_Handle callbackHandle, const char *msg, size_t msgSize);

// Initialize `dart_api_dl.h`
DART_EXPORT intptr_t init(void *data) { return Dart_InitializeApiDL(data); }

DART_EXPORT void register_invoker(CallbackInvoker *invoker) {
  cbInvoker = invoker;
}

DART_EXPORT void register_callback(Dart_Handle callback, const char *id) {
  add_callback(Dart_NewPersistentHandle_DL(callback), id);
}

// Add callback id to invoke
DART_EXPORT void invoke(const char *msg, size_t msgSize, const char *id) {
  CBRegistry *cbEntry;

  HASH_FIND_STR(handles, id, cbEntry);
  if (cbEntry != NULL) {
    Dart_Handle retrievedCBHandle =
        Dart_HandleFromPersistent_DL(cbEntry->handle);
    cbInvoker(retrievedCBHandle, msg, msgSize);
  }
}

// Release all callbacks together
DART_EXPORT void release() {
  CBRegistry *handle, *tmp;
  HASH_ITER(hh, handles, handle, tmp) {
    Dart_DeletePersistentHandle_DL(handle->handle);
    HASH_DEL(handles, handle);
    free(handle);
  }
}
