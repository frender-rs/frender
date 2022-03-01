import { use_state_object } from './snippets/react-sys-a4a9285efecc927b/helpers/use-state.js';

let wasm;

const heap = new Array(32).fill(undefined);

heap.push(undefined, null, true, false);

function getObject(idx) { return heap[idx]; }

let heap_next = heap.length;

function dropObject(idx) {
    if (idx < 36) return;
    heap[idx] = heap_next;
    heap_next = idx;
}

function takeObject(idx) {
    const ret = getObject(idx);
    dropObject(idx);
    return ret;
}

let cachedTextDecoder = new TextDecoder('utf-8', { ignoreBOM: true, fatal: true });

cachedTextDecoder.decode();

let cachegetUint8Memory0 = null;
function getUint8Memory0() {
    if (cachegetUint8Memory0 === null || cachegetUint8Memory0.buffer !== wasm.memory.buffer) {
        cachegetUint8Memory0 = new Uint8Array(wasm.memory.buffer);
    }
    return cachegetUint8Memory0;
}

function getStringFromWasm0(ptr, len) {
    return cachedTextDecoder.decode(getUint8Memory0().subarray(ptr, ptr + len));
}

function addHeapObject(obj) {
    if (heap_next === heap.length) heap.push(heap.length + 1);
    const idx = heap_next;
    heap_next = heap[idx];

    heap[idx] = obj;
    return idx;
}

function debugString(val) {
    // primitive types
    const type = typeof val;
    if (type == 'number' || type == 'boolean' || val == null) {
        return  `${val}`;
    }
    if (type == 'string') {
        return `"${val}"`;
    }
    if (type == 'symbol') {
        const description = val.description;
        if (description == null) {
            return 'Symbol';
        } else {
            return `Symbol(${description})`;
        }
    }
    if (type == 'function') {
        const name = val.name;
        if (typeof name == 'string' && name.length > 0) {
            return `Function(${name})`;
        } else {
            return 'Function';
        }
    }
    // objects
    if (Array.isArray(val)) {
        const length = val.length;
        let debug = '[';
        if (length > 0) {
            debug += debugString(val[0]);
        }
        for(let i = 1; i < length; i++) {
            debug += ', ' + debugString(val[i]);
        }
        debug += ']';
        return debug;
    }
    // Test for built-in
    const builtInMatches = /\[object ([^\]]+)\]/.exec(toString.call(val));
    let className;
    if (builtInMatches.length > 1) {
        className = builtInMatches[1];
    } else {
        // Failed to match the standard '[object ClassName]'
        return toString.call(val);
    }
    if (className == 'Object') {
        // we're a user defined class or Object
        // JSON.stringify avoids problems with cycles, and is generally much
        // easier than looping through ownProperties of `val`.
        try {
            return 'Object(' + JSON.stringify(val) + ')';
        } catch (_) {
            return 'Object';
        }
    }
    // errors
    if (val instanceof Error) {
        return `${val.name}: ${val.message}\n${val.stack}`;
    }
    // TODO we could test for more things here, like `Set`s and `Map`s.
    return className;
}

let WASM_VECTOR_LEN = 0;

let cachedTextEncoder = new TextEncoder('utf-8');

const encodeString = (typeof cachedTextEncoder.encodeInto === 'function'
    ? function (arg, view) {
    return cachedTextEncoder.encodeInto(arg, view);
}
    : function (arg, view) {
    const buf = cachedTextEncoder.encode(arg);
    view.set(buf);
    return {
        read: arg.length,
        written: buf.length
    };
});

function passStringToWasm0(arg, malloc, realloc) {

    if (realloc === undefined) {
        const buf = cachedTextEncoder.encode(arg);
        const ptr = malloc(buf.length);
        getUint8Memory0().subarray(ptr, ptr + buf.length).set(buf);
        WASM_VECTOR_LEN = buf.length;
        return ptr;
    }

    let len = arg.length;
    let ptr = malloc(len);

    const mem = getUint8Memory0();

    let offset = 0;

    for (; offset < len; offset++) {
        const code = arg.charCodeAt(offset);
        if (code > 0x7F) break;
        mem[ptr + offset] = code;
    }

    if (offset !== len) {
        if (offset !== 0) {
            arg = arg.slice(offset);
        }
        ptr = realloc(ptr, len, len = offset + arg.length * 3);
        const view = getUint8Memory0().subarray(ptr + offset, ptr + len);
        const ret = encodeString(arg, view);

        offset += ret.written;
    }

    WASM_VECTOR_LEN = offset;
    return ptr;
}

let cachegetInt32Memory0 = null;
function getInt32Memory0() {
    if (cachegetInt32Memory0 === null || cachegetInt32Memory0.buffer !== wasm.memory.buffer) {
        cachegetInt32Memory0 = new Int32Array(wasm.memory.buffer);
    }
    return cachegetInt32Memory0;
}

function makeClosure(arg0, arg1, dtor, f) {
    const state = { a: arg0, b: arg1, cnt: 1, dtor };
    const real = (...args) => {
        // First up with a closure we increment the internal reference
        // count. This ensures that the Rust closure environment won't
        // be deallocated while we're invoking it.
        state.cnt++;
        try {
            return f(state.a, state.b, ...args);
        } finally {
            if (--state.cnt === 0) {
                wasm.__wbindgen_export_2.get(state.dtor)(state.a, state.b);
                state.a = 0;

            }
        }
    };
    real.original = state;

    return real;
}
function __wbg_adapter_24(arg0, arg1, arg2) {
    var ret = wasm._dyn_core__ops__function__Fn__A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h5fac510e42f3eaa0(arg0, arg1, arg2);
    return ret >>> 0;
}

function __wbg_adapter_27(arg0, arg1) {
    wasm._dyn_core__ops__function__Fn_____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h6dcdccbf10308b32(arg0, arg1);
}

function makeMutClosure(arg0, arg1, dtor, f) {
    const state = { a: arg0, b: arg1, cnt: 1, dtor };
    const real = (...args) => {
        // First up with a closure we increment the internal reference
        // count. This ensures that the Rust closure environment won't
        // be deallocated while we're invoking it.
        state.cnt++;
        const a = state.a;
        state.a = 0;
        try {
            return f(a, state.b, ...args);
        } finally {
            if (--state.cnt === 0) {
                wasm.__wbindgen_export_2.get(state.dtor)(a, state.b);

            } else {
                state.a = a;
            }
        }
    };
    real.original = state;

    return real;
}
function __wbg_adapter_30(arg0, arg1) {
    var ret = wasm._dyn_core__ops__function__FnMut_____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__hd7088c98f7cad901(arg0, arg1);
    return takeObject(ret);
}

function __wbg_adapter_33(arg0, arg1) {
    wasm._dyn_core__ops__function__FnMut_____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h3a4f80385be18aa3(arg0, arg1);
}

function __wbg_adapter_36(arg0, arg1, arg2) {
    var ret = wasm._dyn_core__ops__function__Fn__A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h243e19f47bdc2fa3(arg0, arg1, addHeapObject(arg2));
    return takeObject(ret);
}

let cachegetUint32Memory0 = null;
function getUint32Memory0() {
    if (cachegetUint32Memory0 === null || cachegetUint32Memory0.buffer !== wasm.memory.buffer) {
        cachegetUint32Memory0 = new Uint32Array(wasm.memory.buffer);
    }
    return cachegetUint32Memory0;
}

function passArray32ToWasm0(arg, malloc) {
    const ptr = malloc(arg.length * 4);
    getUint32Memory0().set(arg, ptr / 4);
    WASM_VECTOR_LEN = arg.length;
    return ptr;
}

function getArrayU32FromWasm0(ptr, len) {
    return getUint32Memory0().subarray(ptr / 4, ptr / 4 + len);
}

function isLikeNone(x) {
    return x === undefined || x === null;
}
function __wbg_adapter_59(arg0, arg1) {
    var ret = wasm.wasm_bindgen__convert__closures__invoke0_mut__h8ec668f6a342ca80(arg0, arg1);
    return ret >>> 0;
}

function handleError(f, args) {
    try {
        return f.apply(this, args);
    } catch (e) {
        wasm.__wbindgen_exn_store(addHeapObject(e));
    }
}

async function load(module, imports) {
    if (typeof Response === 'function' && module instanceof Response) {
        if (typeof WebAssembly.instantiateStreaming === 'function') {
            try {
                return await WebAssembly.instantiateStreaming(module, imports);

            } catch (e) {
                if (module.headers.get('Content-Type') != 'application/wasm') {
                    console.warn("`WebAssembly.instantiateStreaming` failed because your server does not serve wasm with `application/wasm` MIME type. Falling back to `WebAssembly.instantiate` which is slower. Original error:\n", e);

                } else {
                    throw e;
                }
            }
        }

        const bytes = await module.arrayBuffer();
        return await WebAssembly.instantiate(bytes, imports);

    } else {
        const instance = await WebAssembly.instantiate(module, imports);

        if (instance instanceof WebAssembly.Instance) {
            return { instance, module };

        } else {
            return instance;
        }
    }
}

async function init(input) {
    if (typeof input === 'undefined') {
        input = new URL('index-a59aa6b6d44cabf2_bg.wasm', import.meta.url);
    }
    const imports = {};
    imports.wbg = {};
    imports.wbg.__wbindgen_cb_drop = function(arg0) {
        const obj = takeObject(arg0).original;
        if (obj.cnt-- == 1) {
            obj.a = 0;
            return true;
        }
        var ret = false;
        return ret;
    };
    imports.wbg.__wbindgen_bigint_new = function(arg0, arg1) {
        var ret = BigInt(getStringFromWasm0(arg0, arg1));
        return addHeapObject(ret);
    };
    imports.wbg.__wbindgen_number_new = function(arg0) {
        var ret = arg0;
        return addHeapObject(ret);
    };
    imports.wbg.__wbindgen_string_new = function(arg0, arg1) {
        var ret = getStringFromWasm0(arg0, arg1);
        return addHeapObject(ret);
    };
    imports.wbg.__wbindgen_is_falsy = function(arg0) {
        var ret = !getObject(arg0);
        return ret;
    };
    imports.wbg.__wbindgen_object_clone_ref = function(arg0) {
        var ret = getObject(arg0);
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_current_d8b62d914c5a4641 = function(arg0, arg1) {
        var ret = getObject(arg1).current;
        var ptr0 = passArray32ToWasm0(ret, wasm.__wbindgen_malloc);
        var len0 = WASM_VECTOR_LEN;
        getInt32Memory0()[arg0 / 4 + 1] = len0;
        getInt32Memory0()[arg0 / 4 + 0] = ptr0;
    };
    imports.wbg.__wbg_setcurrent_832374948e5a3243 = function(arg0, arg1, arg2) {
        var v0 = getArrayU32FromWasm0(arg1, arg2).slice();
        wasm.__wbindgen_free(arg1, arg2 * 4);
        getObject(arg0).current = v0;
    };
    imports.wbg.__wbg_setstate_a6786ff4812ed2d8 = function(arg0, arg1) {
        getObject(arg0).set_state(getObject(arg1));
    };
    imports.wbg.__wbg_frenderPropsBridge_b8f335502af0b984 = function(arg0, arg1) {
        var ret = getObject(arg1).__frenderPropsBridge;
        getInt32Memory0()[arg0 / 4 + 1] = isLikeNone(ret) ? 0 : ret;
        getInt32Memory0()[arg0 / 4 + 0] = !isLikeNone(ret);
    };
    imports.wbg.__wbg_setfrenderPropsBridge_74b00c679d02961d = function(arg0, arg1, arg2) {
        getObject(arg0).__frenderPropsBridge = arg1 === 0 ? undefined : arg2 >>> 0;
    };
    imports.wbg.__wbg_setfrenderDebugComponentName_1ee6536ed9bc9833 = function(arg0, arg1) {
        getObject(arg0).__frenderDebugComponentName = getObject(arg1);
    };
    imports.wbg.__wbg_setfrenderDebugProps_d71e9fb8398d3ff4 = function(arg0, arg1) {
        getObject(arg0).__frenderDebugProps = getObject(arg1);
    };
    imports.wbg.__wbg_setkey_7c41bc1c226e8a9a = function(arg0, arg1) {
        getObject(arg0).key = getObject(arg1);
    };
    imports.wbg.__wbg_children_d04cbc393e51f66e = function(arg0) {
        var ret = getObject(arg0).children;
        return isLikeNone(ret) ? 0 : addHeapObject(ret);
    };
    imports.wbg.__wbindgen_jsval_eq = function(arg0, arg1) {
        var ret = getObject(arg0) === getObject(arg1);
        return ret;
    };
    imports.wbg.__wbg_value_d5cbb6fd225d1499 = function(arg0) {
        var ret = getObject(arg0).value;
        return ret;
    };
    imports.wbg.__wbg_setter_50991ea958826a7a = function(arg0) {
        var ret = getObject(arg0).setter;
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_usestateobject_341dccf0c36102f0 = function(arg0, arg1) {
        try {
            var state0 = {a: arg0, b: arg1};
            var cb0 = () => {
                const a = state0.a;
                state0.a = 0;
                try {
                    return __wbg_adapter_59(a, state0.b, );
                } finally {
                    state0.a = a;
                }
            };
            var ret = use_state_object(cb0);
            return addHeapObject(ret);
        } finally {
            state0.a = state0.b = 0;
        }
    };
    imports.wbg.__wbg_current_45e0f94da2522bc2 = function(arg0, arg1) {
        var ret = getObject(arg1).current;
        getInt32Memory0()[arg0 / 4 + 1] = isLikeNone(ret) ? 0 : ret;
        getInt32Memory0()[arg0 / 4 + 0] = !isLikeNone(ret);
    };
    imports.wbg.__wbg_setcurrent_026834ac98c1e249 = function(arg0, arg1, arg2) {
        getObject(arg0).current = arg1 === 0 ? undefined : arg2 >>> 0;
    };
    imports.wbg.__wbg_useRef_9b9804462d6d080a = function(arg0, arg1) {
        var ret = React.useRef(arg0 === 0 ? undefined : arg1 >>> 0);
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_current_c7d47f724d9928f2 = function(arg0) {
        var ret = getObject(arg0).current;
        return ret;
    };
    imports.wbg.__wbg_setcurrent_3014d72fb235743d = function(arg0, arg1) {
        getObject(arg0).current = arg1 >>> 0;
    };
    imports.wbg.__wbg_useRef_088db9bac8dfbb02 = function(arg0) {
        var ret = React.useRef(arg0 >>> 0);
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_render_893944ba6d37a25f = function(arg0, arg1) {
        ReactDOM.render(getObject(arg0), getObject(arg1));
    };
    imports.wbg.__wbg_createElement_790a7d82250ad948 = function(arg0, arg1) {
        var ret = React.createElement(getObject(arg0), getObject(arg1));
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_createElement_e87c19c1d1892b3c = function(arg0, arg1, arg2) {
        var ret = React.createElement(getObject(arg0), getObject(arg1), ...getObject(arg2));
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_createElement_8b4d1351eefde8a0 = function(arg0, arg1, arg2) {
        var ret = React.createElement(getStringFromWasm0(arg0, arg1), getObject(arg2));
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_createElement_062fa39603e51c69 = function(arg0, arg1, arg2, arg3) {
        var ret = React.createElement(getStringFromWasm0(arg0, arg1), getObject(arg2), ...getObject(arg3));
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_current_2be5e0cef8c3a049 = function(arg0) {
        var ret = getObject(arg0).current;
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_setcurrent_b3d12034f79bece4 = function(arg0, arg1) {
        getObject(arg0).current = takeObject(arg1);
    };
    imports.wbg.__wbg_useRef_6e31f0693af9ef4c = function(arg0) {
        var ret = React.useRef(getObject(arg0));
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_static_accessor_Fragment_0594a3a8bf15ad77 = function() {
        var ret = React.Fragment;
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_static_accessor_StrictMode_e658d7cf46edf55f = function() {
        var ret = React.StrictMode;
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_useEffect_0afa286830e6c24a = function(arg0, arg1) {
        React.useEffect(takeObject(arg0), takeObject(arg1));
    };
    imports.wbg.__wbg_instanceof_Window_434ce1849eb4e0fc = function(arg0) {
        var ret = getObject(arg0) instanceof Window;
        return ret;
    };
    imports.wbg.__wbg_document_5edd43643d1060d9 = function(arg0) {
        var ret = getObject(arg0).document;
        return isLikeNone(ret) ? 0 : addHeapObject(ret);
    };
    imports.wbg.__wbg_clearInterval_ce505853b5064a23 = function(arg0, arg1) {
        getObject(arg0).clearInterval(arg1);
    };
    imports.wbg.__wbg_setInterval_fcb622396840b607 = function() { return handleError(function (arg0, arg1, arg2) {
        var ret = getObject(arg0).setInterval(getObject(arg1), arg2);
        return ret;
    }, arguments) };
    imports.wbg.__wbg_getElementById_b30e88aff96f66a1 = function(arg0, arg1, arg2) {
        var ret = getObject(arg0).getElementById(getStringFromWasm0(arg1, arg2));
        return isLikeNone(ret) ? 0 : addHeapObject(ret);
    };
    imports.wbg.__wbg_error_9caac6ebb9032339 = function(arg0, arg1) {
        console.error(getObject(arg0), getObject(arg1));
    };
    imports.wbg.__wbg_log_fbd13631356d44e4 = function(arg0) {
        console.log(getObject(arg0));
    };
    imports.wbg.__wbindgen_is_undefined = function(arg0) {
        var ret = getObject(arg0) === undefined;
        return ret;
    };
    imports.wbg.__wbg_new_16f24b0728c5e67b = function() {
        var ret = new Array();
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_of_6e090615ff06688d = function(arg0) {
        var ret = Array.of(getObject(arg0));
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_push_a72df856079e6930 = function(arg0, arg1) {
        var ret = getObject(arg0).push(getObject(arg1));
        return ret;
    };
    imports.wbg.__wbg_toString_438e03f57fe1aad9 = function() { return handleError(function (arg0, arg1) {
        var ret = getObject(arg0).toString(arg1);
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_newnoargs_f579424187aa1717 = function(arg0, arg1) {
        var ret = new Function(getStringFromWasm0(arg0, arg1));
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_call_89558c3e96703ca1 = function() { return handleError(function (arg0, arg1) {
        var ret = getObject(arg0).call(getObject(arg1));
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_new_d3138911a89329b0 = function() {
        var ret = new Object();
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_globalThis_d61b1f48a57191ae = function() { return handleError(function () {
        var ret = globalThis.globalThis;
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_self_e23d74ae45fb17d1 = function() { return handleError(function () {
        var ret = self.self;
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_window_b4be7f48b24ac56e = function() { return handleError(function () {
        var ret = window.window;
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_global_e7669da72fd7f239 = function() { return handleError(function () {
        var ret = global.global;
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbindgen_object_drop_ref = function(arg0) {
        takeObject(arg0);
    };
    imports.wbg.__wbg_deleteProperty_afde0e7d23b40601 = function() { return handleError(function (arg0, arg1) {
        var ret = Reflect.deleteProperty(getObject(arg0), getObject(arg1));
        return ret;
    }, arguments) };
    imports.wbg.__wbg_set_c42875065132a932 = function() { return handleError(function (arg0, arg1, arg2) {
        var ret = Reflect.set(getObject(arg0), getObject(arg1), getObject(arg2));
        return ret;
    }, arguments) };
    imports.wbg.__wbindgen_debug_string = function(arg0, arg1) {
        var ret = debugString(getObject(arg1));
        var ptr0 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len0 = WASM_VECTOR_LEN;
        getInt32Memory0()[arg0 / 4 + 1] = len0;
        getInt32Memory0()[arg0 / 4 + 0] = ptr0;
    };
    imports.wbg.__wbindgen_throw = function(arg0, arg1) {
        throw new Error(getStringFromWasm0(arg0, arg1));
    };
    imports.wbg.__wbindgen_closure_wrapper444 = function(arg0, arg1, arg2) {
        var ret = makeClosure(arg0, arg1, 128, __wbg_adapter_24);
        return addHeapObject(ret);
    };
    imports.wbg.__wbindgen_closure_wrapper446 = function(arg0, arg1, arg2) {
        var ret = makeClosure(arg0, arg1, 124, __wbg_adapter_27);
        return addHeapObject(ret);
    };
    imports.wbg.__wbindgen_closure_wrapper1929 = function(arg0, arg1, arg2) {
        var ret = makeMutClosure(arg0, arg1, 201, __wbg_adapter_30);
        return addHeapObject(ret);
    };
    imports.wbg.__wbindgen_closure_wrapper1931 = function(arg0, arg1, arg2) {
        var ret = makeMutClosure(arg0, arg1, 199, __wbg_adapter_33);
        return addHeapObject(ret);
    };
    imports.wbg.__wbindgen_closure_wrapper1933 = function(arg0, arg1, arg2) {
        var ret = makeClosure(arg0, arg1, 203, __wbg_adapter_36);
        return addHeapObject(ret);
    };

    if (typeof input === 'string' || (typeof Request === 'function' && input instanceof Request) || (typeof URL === 'function' && input instanceof URL)) {
        input = fetch(input);
    }



    const { instance, module } = await load(await input, imports);

    wasm = instance.exports;
    init.__wbindgen_wasm_module = module;
    wasm.__wbindgen_start();
    return wasm;
}

export default init;

