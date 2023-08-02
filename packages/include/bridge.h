#ifndef __libwaku_dart_bridge__
#define __libwaku_dart_bridge__

#include "headers/dart_api.h"

#include <stddef.h>

typedef void(CallbackInvoker)(Dart_Handle callbackHandle, const char *msg,
                              size_t msgSize);

DART_EXPORT intptr_t init(void *data);

DART_EXPORT void register_invoker(CallbackInvoker *invoker);

DART_EXPORT void register_callback(Dart_Handle callback, const char *id);

DART_EXPORT void invoke(const char *msg, size_t msgSize, const char *id);

DART_EXPORT void release();

#endif /*__libwaku_dart_bridge__*/
