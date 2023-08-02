#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>

using CallbackInvoker = void(*)(Dart_Handle handle, CString msg, uintptr_t msgSize);

extern "C" {

intptr_t init(void *data);

void register_invoker(const CallbackInvoker *invoker);

void register(Dart_Handle cb, const CString *id);

void invoke(const CString *msg, uintptr_t msg_len, const CString *id);

void release();

} // extern "C"
