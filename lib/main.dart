// ignore_for_file: public_member_api_docs

import 'dart:ffi' as ffi;
import 'dart:io' show Platform;
import 'dart:math';

import 'package:ffi/ffi.dart';
import 'package:flutter/foundation.dart';
import 'package:flutter/material.dart';
import 'package:path/path.dart';

import 'src/waku_dart_bridge_bindings.dart';

typedef CString = ffi.Pointer<ffi.Char>;
typedef NativeCBRegistrar = ffi.Void Function(
  ffi.Handle callback,
  CString msg,
  ffi.Size msgLen,
);
typedef WakuCallback = void Function(String msg, int msgLen);

/// Extension on `Pointer<Char>` to convert to a Dart string.
extension _ToDartString on CString {
  /// Converts the [ffi.Pointer] to a Dart string.
  String toDartString() => cast<Utf8>().toDartString();
}

/// Extension to convert Dart objects to C string pointers.
extension _ToCString on Object {
  /// Converts the object to a null-terminated C string pointer.
  CString toCStr() => toString().toNativeUtf8().cast<ffi.Char>();
}

const String dir = 'dart_bridge';
final ffi.DynamicLibrary lib = _openDynamicLibrary(_join(dir), 'waku_$dir');
final WakuDartBridge bridge = WakuDartBridge(lib);

void invoker(final Object callback, final CString msg, final int msgLen) =>
    (callback as WakuCallback)(msg.toDartString(), msgLen);

void main() {
  bridge
    ..init(ffi.NativeApi.initializeApiDLData)
    ..register_invoker(ffi.Pointer.fromFunction<NativeCBRegistrar>(invoker));

  runApp(const MyApp());
}

class MyApp extends StatelessWidget {
  const MyApp({super.key});

  @override
  Widget build(final BuildContext context) => MaterialApp(
        title: 'Flutter Demo',
        theme: ThemeData(
          colorScheme: ColorScheme.fromSeed(seedColor: Colors.deepPurple),
          useMaterial3: true,
        ),
        home: const MyHomePage(title: 'Flutter Demo Home Page'),
      );
}

class MyHomePage extends StatefulWidget {
  const MyHomePage({required this.title, super.key});

  final String title;

  @override
  State<MyHomePage> createState() => _MyHomePageState();

  @override
  void debugFillProperties(final DiagnosticPropertiesBuilder properties) {
    super.debugFillProperties(properties);
    properties.add(StringProperty('title', title));
  }
}

class _MyHomePageState extends State<MyHomePage> {
  int _counter = 0;

  void _incrementCounter(final String msg, final int msgLen) {
    try {
      setState(() {
        final int other = (int.parse(msg) + msgLen).abs();
        if (kDebugMode) {
          print('_incrementCounter');
        }
        _counter += other + Random().nextInt(10);
      });
    } on Exception catch (e, s) {
      if (kDebugMode) {
        print('Exception: $e\nStacktrace: $s');
      }
    }
  }

  void _decrementCounter(final String msg, final int msgLen) {
    try {
      setState(() {
        final int other = (int.parse(msg) + msgLen).abs();
        if (kDebugMode) {
          print('_decrementCounter');
        }
        _counter += -other - Random().nextInt(10);
      });
    } on Exception catch (e, s) {
      if (kDebugMode) {
        print('Exception: $e\nStacktrace: $s');
      }
    }
  }

  @override
  void initState() {
    super.initState();
    bridge
      ..register_callback(_incrementCounter, 'increment'.toCStr())
      ..register_callback(_decrementCounter, 'decrement'.toCStr());
  }

  @override
  void dispose() {
    bridge.release();
    super.dispose();
  }

  @override
  Widget build(final BuildContext context) => Scaffold(
        appBar: AppBar(
          backgroundColor: Theme.of(context).colorScheme.inversePrimary,
          title: Text(widget.title),
        ),
        body: Center(
          child: Column(
            mainAxisAlignment: MainAxisAlignment.center,
            children: <Widget>[
              const Text(
                'You have pushed the button this many times:',
              ),
              Text(
                '$_counter',
                style: Theme.of(context).textTheme.headlineMedium,
              ),
              Row(
                children: <Widget>[
                  ElevatedButton(
                    onPressed: () {
                      bridge.invoke(
                        _counter.toCStr(),
                        _counter.toString().length,
                        'decrement'.toCStr(),
                      );
                    },
                    child: const Icon(Icons.remove),
                  ),
                  const Spacer(),
                  ElevatedButton(
                    onPressed: () {
                      bridge.invoke(
                        _counter.toCStr(),
                        _counter.toString().length,
                        'increment'.toCStr(),
                      );
                    },
                    child: const Icon(Icons.add),
                  )
                ],
              )
            ],
          ),
        ),
      );
}

ffi.DynamicLibrary _openDynamicLibrary(final String out, final String libname) {
  late final String path;
  if (Platform.isIOS) {
    path = '$libname.framework/$libname';
  } else if (Platform.isAndroid || Platform.isLinux || Platform.isFuchsia) {
    path = 'lib$libname.so';
  } else if (Platform.isMacOS) {
    path = 'lib$libname.dylib';
  } else if (Platform.isWindows) {
    path = '$libname.dll';
  } else {
    throw UnsupportedError('Unknown platform: ${Platform.operatingSystem}');
  }

  return ffi.DynamicLibrary.open(out + path);
}

String _join(final String subfolder) => 'packages$separator$subfolder'
    '${separator}target${separator}release$separator';
