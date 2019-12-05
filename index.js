import * as wasm from './index_bg.wasm';

function __wbg_elem_binding0(arg0, arg1) {
    wasm.__wbg_function_table.get(9)(arg0, arg1);
}
/**
*/
export function main_js() {
    wasm.main_js();
}

const heap = new Array(32);

heap.fill(undefined);

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

function addHeapObject(obj) {
    if (heap_next === heap.length) heap.push(heap.length + 1);
    const idx = heap_next;
    heap_next = heap[idx];

    heap[idx] = obj;
    return idx;
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

let cachegetUint8Memory = null;
function getUint8Memory() {
    if (cachegetUint8Memory === null || cachegetUint8Memory.buffer !== wasm.memory.buffer) {
        cachegetUint8Memory = new Uint8Array(wasm.memory.buffer);
    }
    return cachegetUint8Memory;
}

function passStringToWasm(arg) {

    let len = arg.length;
    let ptr = wasm.__wbindgen_malloc(len);

    const mem = getUint8Memory();

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
        ptr = wasm.__wbindgen_realloc(ptr, len, len = offset + arg.length * 3);
        const view = getUint8Memory().subarray(ptr + offset, ptr + len);
        const ret = encodeString(arg, view);

        offset += ret.written;
    }

    WASM_VECTOR_LEN = offset;
    return ptr;
}

let cachegetInt32Memory = null;
function getInt32Memory() {
    if (cachegetInt32Memory === null || cachegetInt32Memory.buffer !== wasm.memory.buffer) {
        cachegetInt32Memory = new Int32Array(wasm.memory.buffer);
    }
    return cachegetInt32Memory;
}

let cachedTextDecoder = new TextDecoder('utf-8', { ignoreBOM: true, fatal: true });

cachedTextDecoder.decode();

function getStringFromWasm(ptr, len) {
    return cachedTextDecoder.decode(getUint8Memory().subarray(ptr, ptr + len));
}

function isLikeNone(x) {
    return x === undefined || x === null;
}

function handleError(e) {
    wasm.__wbindgen_exn_store(addHeapObject(e));
}

function notDefined(what) { return () => { throw new Error(`${what} is not defined`); }; }

function getArrayU8FromWasm(ptr, len) {
    return getUint8Memory().subarray(ptr / 1, ptr / 1 + len);
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

export const __wbindgen_object_drop_ref = function(arg0) {
    takeObject(arg0);
};

export const __wbindgen_cb_forget = function(arg0) {
    takeObject(arg0);
};

export const __wbindgen_cb_drop = function(arg0) {
    const obj = takeObject(arg0).original;
    if (obj.cnt-- == 1) {
        obj.a = 0;
        return true;
    }
    const ret = false;
    return ret;
};

export const __wbg_new_59cb74e423758ede = function() {
    const ret = new Error();
    return addHeapObject(ret);
};

export const __wbg_stack_558ba5917b466edd = function(arg0, arg1) {
    const ret = getObject(arg1).stack;
    const ret0 = passStringToWasm(ret);
    const ret1 = WASM_VECTOR_LEN;
    getInt32Memory()[arg0 / 4 + 0] = ret0;
    getInt32Memory()[arg0 / 4 + 1] = ret1;
};

export const __wbg_error_4bb6c2a97407129a = function(arg0, arg1) {
    const v0 = getStringFromWasm(arg0, arg1).slice();
    wasm.__wbindgen_free(arg0, arg1 * 1);
    console.error(v0);
};

export const __widl_instanceof_Window = function(arg0) {
    const ret = getObject(arg0) instanceof Window;
    return ret;
};

export const __widl_instanceof_CanvasRenderingContext2D = function(arg0) {
    const ret = getObject(arg0) instanceof CanvasRenderingContext2D;
    return ret;
};

export const __widl_f_set_fill_style_CanvasRenderingContext2D = function(arg0, arg1) {
    getObject(arg0).fillStyle = getObject(arg1);
};

export const __widl_f_clear_rect_CanvasRenderingContext2D = function(arg0, arg1, arg2, arg3, arg4) {
    getObject(arg0).clearRect(arg1, arg2, arg3, arg4);
};

export const __widl_f_fill_rect_CanvasRenderingContext2D = function(arg0, arg1, arg2, arg3, arg4) {
    getObject(arg0).fillRect(arg1, arg2, arg3, arg4);
};

export const __widl_f_get_element_by_id_Document = function(arg0, arg1, arg2) {
    const ret = getObject(arg0).getElementById(getStringFromWasm(arg1, arg2));
    return isLikeNone(ret) ? 0 : addHeapObject(ret);
};

export const __widl_f_client_width_Element = function(arg0) {
    const ret = getObject(arg0).clientWidth;
    return ret;
};

export const __widl_f_client_height_Element = function(arg0) {
    const ret = getObject(arg0).clientHeight;
    return ret;
};

export const __widl_instanceof_HTMLButtonElement = function(arg0) {
    const ret = getObject(arg0) instanceof HTMLButtonElement;
    return ret;
};

export const __widl_instanceof_HTMLCanvasElement = function(arg0) {
    const ret = getObject(arg0) instanceof HTMLCanvasElement;
    return ret;
};

export const __widl_f_get_context_HTMLCanvasElement = function(arg0, arg1, arg2) {
    try {
        const ret = getObject(arg0).getContext(getStringFromWasm(arg1, arg2));
        return isLikeNone(ret) ? 0 : addHeapObject(ret);
    } catch (e) {
        handleError(e)
    }
};

export const __widl_f_width_HTMLCanvasElement = function(arg0) {
    const ret = getObject(arg0).width;
    return ret;
};

export const __widl_f_set_width_HTMLCanvasElement = function(arg0, arg1) {
    getObject(arg0).width = arg1 >>> 0;
};

export const __widl_f_height_HTMLCanvasElement = function(arg0) {
    const ret = getObject(arg0).height;
    return ret;
};

export const __widl_f_set_height_HTMLCanvasElement = function(arg0, arg1) {
    getObject(arg0).height = arg1 >>> 0;
};

export const __widl_f_set_onchange_HTMLElement = function(arg0, arg1) {
    getObject(arg0).onchange = getObject(arg1);
};

export const __widl_f_set_onclick_HTMLElement = function(arg0, arg1) {
    getObject(arg0).onclick = getObject(arg1);
};

export const __widl_instanceof_HTMLInputElement = function(arg0) {
    const ret = getObject(arg0) instanceof HTMLInputElement;
    return ret;
};

export const __widl_f_value_HTMLInputElement = function(arg0, arg1) {
    const ret = getObject(arg1).value;
    const ret0 = passStringToWasm(ret);
    const ret1 = WASM_VECTOR_LEN;
    getInt32Memory()[arg0 / 4 + 0] = ret0;
    getInt32Memory()[arg0 / 4 + 1] = ret1;
};

export const __widl_instanceof_HTMLSelectElement = function(arg0) {
    const ret = getObject(arg0) instanceof HTMLSelectElement;
    return ret;
};

export const __widl_f_value_HTMLSelectElement = function(arg0, arg1) {
    const ret = getObject(arg1).value;
    const ret0 = passStringToWasm(ret);
    const ret1 = WASM_VECTOR_LEN;
    getInt32Memory()[arg0 / 4 + 0] = ret0;
    getInt32Memory()[arg0 / 4 + 1] = ret1;
};

export const __widl_f_request_animation_frame_Window = function(arg0, arg1) {
    try {
        const ret = getObject(arg0).requestAnimationFrame(getObject(arg1));
        return ret;
    } catch (e) {
        handleError(e)
    }
};

export const __widl_f_document_Window = function(arg0) {
    const ret = getObject(arg0).document;
    return isLikeNone(ret) ? 0 : addHeapObject(ret);
};

export const __widl_f_set_onresize_Window = function(arg0, arg1) {
    getObject(arg0).onresize = getObject(arg1);
};

export const __wbindgen_string_new = function(arg0, arg1) {
    const ret = getStringFromWasm(arg0, arg1);
    return addHeapObject(ret);
};

export const __wbg_newnoargs_0c3c518a7f7c56bf = function(arg0, arg1) {
    const ret = new Function(getStringFromWasm(arg0, arg1));
    return addHeapObject(ret);
};

export const __wbg_call_aa56d0132fec7569 = function(arg0, arg1) {
    try {
        const ret = getObject(arg0).call(getObject(arg1));
        return addHeapObject(ret);
    } catch (e) {
        handleError(e)
    }
};

export const __wbg_globalThis_4fa2faeae7a7a380 = function() {
    try {
        const ret = globalThis.globalThis;
        return addHeapObject(ret);
    } catch (e) {
        handleError(e)
    }
};

export const __wbg_self_ed02073ec1d8fef4 = function() {
    try {
        const ret = self.self;
        return addHeapObject(ret);
    } catch (e) {
        handleError(e)
    }
};

export const __wbg_window_356847be61f4a80f = function() {
    try {
        const ret = window.window;
        return addHeapObject(ret);
    } catch (e) {
        handleError(e)
    }
};

export const __wbg_global_6580a67633b0dbc1 = function() {
    try {
        const ret = global.global;
        return addHeapObject(ret);
    } catch (e) {
        handleError(e)
    }
};

export const __wbindgen_is_undefined = function(arg0) {
    const ret = getObject(arg0) === undefined;
    return ret;
};

export const __wbindgen_object_clone_ref = function(arg0) {
    const ret = getObject(arg0);
    return addHeapObject(ret);
};

export const __wbg_ceil_4bb389d11937eaba = typeof Math.ceil == 'function' ? Math.ceil : notDefined('Math.ceil');

export const __wbg_floor_2b7168c7e4c4dcca = typeof Math.floor == 'function' ? Math.floor : notDefined('Math.floor');

export const __wbg_randomFillSync_eabbc18af655bfbe = function(arg0, arg1, arg2) {
    getObject(arg0).randomFillSync(getArrayU8FromWasm(arg1, arg2));
};

export const __wbg_getRandomValues_40ceff860009fa55 = function(arg0, arg1, arg2) {
    getObject(arg0).getRandomValues(getArrayU8FromWasm(arg1, arg2));
};

export const __wbg_self_e70540c4956ad879 = function() {
    try {
        const ret = self.self;
        return addHeapObject(ret);
    } catch (e) {
        handleError(e)
    }
};

export const __wbg_require_9edeecb69c9dc31c = function(arg0, arg1) {
    const ret = require(getStringFromWasm(arg0, arg1));
    return addHeapObject(ret);
};

export const __wbg_crypto_58b0c631995fea92 = function(arg0) {
    const ret = getObject(arg0).crypto;
    return addHeapObject(ret);
};

export const __wbg_getRandomValues_532ec62d8e780edc = function(arg0) {
    const ret = getObject(arg0).getRandomValues;
    return addHeapObject(ret);
};

export const __wbindgen_debug_string = function(arg0, arg1) {
    const ret = debugString(getObject(arg1));
    const ret0 = passStringToWasm(ret);
    const ret1 = WASM_VECTOR_LEN;
    getInt32Memory()[arg0 / 4 + 0] = ret0;
    getInt32Memory()[arg0 / 4 + 1] = ret1;
};

export const __wbindgen_throw = function(arg0, arg1) {
    throw new Error(getStringFromWasm(arg0, arg1));
};

export const __wbindgen_closure_wrapper139 = function(arg0, arg1, arg2) {
    const state = { a: arg0, b: arg1, cnt: 1 };
    const real = () => {
        state.cnt++;
        const a = state.a;
        state.a = 0;
        try {
            return __wbg_elem_binding0(a, state.b, );
        } finally {
            if (--state.cnt === 0) wasm.__wbg_function_table.get(10)(a, state.b);
            else state.a = a;
        }
    }
    ;
    real.original = state;
    const ret = real;
    return addHeapObject(ret);
};

wasm.__wbindgen_start();

