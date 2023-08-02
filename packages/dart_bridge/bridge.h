#ifndef __libwaku_dart_bridge__
#define __libwaku_dart_bridge__

#include "include/dart_api.h"
#include "include/dart_api_dl.h"
#include "include/dart_native_api.h"

#include <stddef.h>

typedef void(CallbackInvoker)(Dart_Handle callback, const char *msg,
                              size_t msgLen);

DART_EXPORT intptr_t init(void *data);

DART_EXPORT void registerCBInvoker(CallbackInvoker *invoker);

DART_EXPORT void registerCB(Dart_Handle callback, const char *id);

DART_EXPORT void invokeCB(const char *msg, size_t msgLen, const char *id);

DART_EXPORT void releaseCB();

#endif /*__libwaku_dart_bridge__*/
