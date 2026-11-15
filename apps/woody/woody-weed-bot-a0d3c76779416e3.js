import { RawInterpreter } from './snippets/dioxus-interpreter-js-53424282ea002554/inline0.js';
import { setAttributeInner } from './snippets/dioxus-interpreter-js-53424282ea002554/src/js/common.js';
import { get_form_data } from './snippets/dioxus-web-bf44d47c344f35d0/inline0.js';
import { get_select_data } from './snippets/dioxus-web-bf44d47c344f35d0/inline1.js';
import { WebDioxusChannel } from './snippets/dioxus-web-bf44d47c344f35d0/src/js/eval.js';


export class JSOwner {
    static __wrap(ptr) {
        const obj = Object.create(JSOwner.prototype);
        obj.__wbg_ptr = ptr;
        JSOwnerFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }
    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        JSOwnerFinalization.unregister(this);
        return ptr;
    }
    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_jsowner_free(ptr, 0);
    }
}
if (Symbol.dispose) JSOwner.prototype[Symbol.dispose] = JSOwner.prototype.free;

export function run() {
    wasm.run();
}
function __wbg_get_imports() {
    const import0 = {
        __proto__: null,
        __wbg_Error_ef53bc310eb298a0: function(arg0, arg1) {
            const ret = Error(getStringFromWasm0(arg0, arg1));
            return addHeapObject(ret);
        },
        __wbg_String_b51de6b05a10845b: function(arg0, arg1) {
            const ret = String(getObject(arg1));
            const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_export, wasm.__wbindgen_export2);
            const len1 = WASM_VECTOR_LEN;
            getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
            getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
        },
        __wbg___wbindgen_bigint_get_as_i64_38130e98eecd467d: function(arg0, arg1) {
            const v = getObject(arg1);
            const ret = typeof(v) === 'bigint' ? v : undefined;
            getDataViewMemory0().setBigInt64(arg0 + 8 * 1, isLikeNone(ret) ? BigInt(0) : ret, true);
            getDataViewMemory0().setInt32(arg0 + 4 * 0, !isLikeNone(ret), true);
        },
        __wbg___wbindgen_boolean_get_1a45e2c38d4d41b9: function(arg0) {
            const v = getObject(arg0);
            const ret = typeof(v) === 'boolean' ? v : undefined;
            return isLikeNone(ret) ? 0xFFFFFF : ret ? 1 : 0;
        },
        __wbg___wbindgen_debug_string_0accd80f45e5faa2: function(arg0, arg1) {
            const ret = debugString(getObject(arg1));
            const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_export, wasm.__wbindgen_export2);
            const len1 = WASM_VECTOR_LEN;
            getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
            getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
        },
        __wbg___wbindgen_in_70a403a56e771704: function(arg0, arg1) {
            const ret = getObject(arg0) in getObject(arg1);
            return ret;
        },
        __wbg___wbindgen_is_bigint_6ffd6468a9bc44b9: function(arg0) {
            const ret = typeof(getObject(arg0)) === 'bigint';
            return ret;
        },
        __wbg___wbindgen_is_function_754e9f305ff6029e: function(arg0) {
            const ret = typeof(getObject(arg0)) === 'function';
            return ret;
        },
        __wbg___wbindgen_is_null_87c3bfe968c6a5ad: function(arg0) {
            const ret = getObject(arg0) === null;
            return ret;
        },
        __wbg___wbindgen_is_object_56732c2bc353f41d: function(arg0) {
            const val = getObject(arg0);
            const ret = typeof(val) === 'object' && val !== null;
            return ret;
        },
        __wbg___wbindgen_is_string_c236cabd84a4d769: function(arg0) {
            const ret = typeof(getObject(arg0)) === 'string';
            return ret;
        },
        __wbg___wbindgen_is_undefined_67b456be8673d3d7: function(arg0) {
            const ret = getObject(arg0) === undefined;
            return ret;
        },
        __wbg___wbindgen_jsval_eq_1068e624fa87f6ab: function(arg0, arg1) {
            const ret = getObject(arg0) === getObject(arg1);
            return ret;
        },
        __wbg___wbindgen_jsval_loose_eq_2c56564c75129511: function(arg0, arg1) {
            const ret = getObject(arg0) == getObject(arg1);
            return ret;
        },
        __wbg___wbindgen_memory_fbc4c3e30b409f08: function() {
            const ret = wasm.memory;
            return addHeapObject(ret);
        },
        __wbg___wbindgen_number_get_9bb1761122181af2: function(arg0, arg1) {
            const obj = getObject(arg1);
            const ret = typeof(obj) === 'number' ? obj : undefined;
            getDataViewMemory0().setFloat64(arg0 + 8 * 1, isLikeNone(ret) ? 0 : ret, true);
            getDataViewMemory0().setInt32(arg0 + 4 * 0, !isLikeNone(ret), true);
        },
        __wbg___wbindgen_string_get_72bdf95d3ae505b1: function(arg0, arg1) {
            const obj = getObject(arg1);
            const ret = typeof(obj) === 'string' ? obj : undefined;
            var ptr1 = isLikeNone(ret) ? 0 : passStringToWasm0(ret, wasm.__wbindgen_export, wasm.__wbindgen_export2);
            var len1 = WASM_VECTOR_LEN;
            getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
            getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
        },
        __wbg___wbindgen_throw_1506f2235d1bdba0: function(arg0, arg1) {
            throw new Error(getStringFromWasm0(arg0, arg1));
        },
        __wbg__wbg_cb_unref_61db23ac97f16c31: function(arg0) {
            getObject(arg0)._wbg_cb_unref();
        },
        __wbg_addEventListener_5593b0efd622abd6: function() { return handleError(function (arg0, arg1, arg2, arg3, arg4) {
            getObject(arg0).addEventListener(getStringFromWasm0(arg1, arg2), getObject(arg3), getObject(arg4));
        }, arguments); },
        __wbg_addEventListener_7c5a0db2b2826a06: function() { return handleError(function (arg0, arg1, arg2, arg3) {
            getObject(arg0).addEventListener(getStringFromWasm0(arg1, arg2), getObject(arg3));
        }, arguments); },
        __wbg_altKey_4efe9bf67d712839: function(arg0) {
            const ret = getObject(arg0).altKey;
            return ret;
        },
        __wbg_altKey_77d5df8df54f3c7e: function(arg0) {
            const ret = getObject(arg0).altKey;
            return ret;
        },
        __wbg_altKey_f112365f48461ccc: function(arg0) {
            const ret = getObject(arg0).altKey;
            return ret;
        },
        __wbg_animationName_23e8befb93220b3c: function(arg0, arg1) {
            const ret = getObject(arg1).animationName;
            const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_export, wasm.__wbindgen_export2);
            const len1 = WASM_VECTOR_LEN;
            getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
            getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
        },
        __wbg_appendChild_364435158a70bd03: function() { return handleError(function (arg0, arg1) {
            const ret = getObject(arg0).appendChild(getObject(arg1));
            return addHeapObject(ret);
        }, arguments); },
        __wbg_back_3e53b5980d9fb5bd: function() { return handleError(function (arg0) {
            getObject(arg0).back();
        }, arguments); },
        __wbg_blockSize_4831f37080060d97: function(arg0) {
            const ret = getObject(arg0).blockSize;
            return ret;
        },
        __wbg_blur_fa177adf4bb4d43a: function() { return handleError(function (arg0) {
            getObject(arg0).blur();
        }, arguments); },
        __wbg_body_7d5b1a2ac7f2c821: function(arg0) {
            const ret = getObject(arg0).body;
            return isLikeNone(ret) ? 0 : addHeapObject(ret);
        },
        __wbg_borderBoxSize_39a36ccf939c950d: function(arg0) {
            const ret = getObject(arg0).borderBoxSize;
            return addHeapObject(ret);
        },
        __wbg_boundingClientRect_cc52b11bfb9da8cd: function(arg0) {
            const ret = getObject(arg0).boundingClientRect;
            return addHeapObject(ret);
        },
        __wbg_bubbles_6507901b016ba2aa: function(arg0) {
            const ret = getObject(arg0).bubbles;
            return ret;
        },
        __wbg_button_f3dc4c82e6ee9a0c: function(arg0) {
            const ret = getObject(arg0).button;
            return ret;
        },
        __wbg_buttons_8dae14f7d9ea8c8a: function(arg0) {
            const ret = getObject(arg0).buttons;
            return ret;
        },
        __wbg_call_8a89609d89f6608a: function() { return handleError(function (arg0, arg1) {
            const ret = getObject(arg0).call(getObject(arg1));
            return addHeapObject(ret);
        }, arguments); },
        __wbg_call_9c758de292015997: function() { return handleError(function (arg0, arg1, arg2) {
            const ret = getObject(arg0).call(getObject(arg1), getObject(arg2));
            return addHeapObject(ret);
        }, arguments); },
        __wbg_changedTouches_f4bc81aa9275b3b1: function(arg0) {
            const ret = getObject(arg0).changedTouches;
            return addHeapObject(ret);
        },
        __wbg_charCodeAt_464da5455a6116e4: function(arg0, arg1) {
            const ret = getObject(arg0).charCodeAt(arg1 >>> 0);
            return ret;
        },
        __wbg_checked_08fc50bf12676638: function(arg0) {
            const ret = getObject(arg0).checked;
            return ret;
        },
        __wbg_clearTimeout_113b1cde814ec762: function(arg0) {
            const ret = clearTimeout(takeObject(arg0));
            return addHeapObject(ret);
        },
        __wbg_click_f4ad50d5e13b83d8: function(arg0) {
            getObject(arg0).click();
        },
        __wbg_clientX_6a613cbcceb44e9d: function(arg0) {
            const ret = getObject(arg0).clientX;
            return ret;
        },
        __wbg_clientX_c85019015e605e82: function(arg0) {
            const ret = getObject(arg0).clientX;
            return ret;
        },
        __wbg_clientY_48b9212040ca1133: function(arg0) {
            const ret = getObject(arg0).clientY;
            return ret;
        },
        __wbg_clientY_e89b1cdbdb6c1772: function(arg0) {
            const ret = getObject(arg0).clientY;
            return ret;
        },
        __wbg_clipboard_53ba93c4ad802c3d: function(arg0) {
            const ret = getObject(arg0).clipboard;
            return addHeapObject(ret);
        },
        __wbg_code_fda4a2b9044681ac: function(arg0, arg1) {
            const ret = getObject(arg1).code;
            const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_export, wasm.__wbindgen_export2);
            const len1 = WASM_VECTOR_LEN;
            getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
            getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
        },
        __wbg_contentBoxSize_48fabcd0eea7c23e: function(arg0) {
            const ret = getObject(arg0).contentBoxSize;
            return addHeapObject(ret);
        },
        __wbg_createComment_fee3099b63c574fb: function(arg0, arg1, arg2) {
            const ret = getObject(arg0).createComment(getStringFromWasm0(arg1, arg2));
            return addHeapObject(ret);
        },
        __wbg_createElementNS_2c964c61db671f5d: function() { return handleError(function (arg0, arg1, arg2, arg3, arg4) {
            const ret = getObject(arg0).createElementNS(arg1 === 0 ? undefined : getStringFromWasm0(arg1, arg2), getStringFromWasm0(arg3, arg4));
            return addHeapObject(ret);
        }, arguments); },
        __wbg_createElement_c3c16a9aa7f5cc74: function() { return handleError(function (arg0, arg1, arg2) {
            const ret = getObject(arg0).createElement(getStringFromWasm0(arg1, arg2));
            return addHeapObject(ret);
        }, arguments); },
        __wbg_createObjectURL_395ba916655726cd: function() { return handleError(function (arg0, arg1) {
            const ret = URL.createObjectURL(getObject(arg1));
            const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_export, wasm.__wbindgen_export2);
            const len1 = WASM_VECTOR_LEN;
            getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
            getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
        }, arguments); },
        __wbg_createTextNode_f78a04409331196b: function(arg0, arg1, arg2) {
            const ret = getObject(arg0).createTextNode(getStringFromWasm0(arg1, arg2));
            return addHeapObject(ret);
        },
        __wbg_ctrlKey_1cae6780759a470d: function(arg0) {
            const ret = getObject(arg0).ctrlKey;
            return ret;
        },
        __wbg_ctrlKey_9a3be9ca366f0d73: function(arg0) {
            const ret = getObject(arg0).ctrlKey;
            return ret;
        },
        __wbg_ctrlKey_a1ca4695e4fe525a: function(arg0) {
            const ret = getObject(arg0).ctrlKey;
            return ret;
        },
        __wbg_dataTransfer_bf723904983b1674: function(arg0) {
            const ret = getObject(arg0).dataTransfer;
            return isLikeNone(ret) ? 0 : addHeapObject(ret);
        },
        __wbg_data_8d4459e190fc1615: function(arg0, arg1) {
            const ret = getObject(arg1).data;
            var ptr1 = isLikeNone(ret) ? 0 : passStringToWasm0(ret, wasm.__wbindgen_export, wasm.__wbindgen_export2);
            var len1 = WASM_VECTOR_LEN;
            getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
            getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
        },
        __wbg_deltaMode_40f5de10d348f7e8: function(arg0) {
            const ret = getObject(arg0).deltaMode;
            return ret;
        },
        __wbg_deltaX_5193430f52fdad3f: function(arg0) {
            const ret = getObject(arg0).deltaX;
            return ret;
        },
        __wbg_deltaY_e1fefa22823d87c4: function(arg0) {
            const ret = getObject(arg0).deltaY;
            return ret;
        },
        __wbg_deltaZ_0cd74e7cc4f80662: function(arg0) {
            const ret = getObject(arg0).deltaZ;
            return ret;
        },
        __wbg_detail_8d603e8cc69062d8: function(arg0) {
            const ret = getObject(arg0).detail;
            return addHeapObject(ret);
        },
        __wbg_documentElement_19b770919f3ea991: function(arg0) {
            const ret = getObject(arg0).documentElement;
            return isLikeNone(ret) ? 0 : addHeapObject(ret);
        },
        __wbg_document_aceb08cd6489baf5: function(arg0) {
            const ret = getObject(arg0).document;
            return isLikeNone(ret) ? 0 : addHeapObject(ret);
        },
        __wbg_done_60cf307fcc680536: function(arg0) {
            const ret = getObject(arg0).done;
            return ret;
        },
        __wbg_elapsedTime_134de65ccf47aee6: function(arg0) {
            const ret = getObject(arg0).elapsedTime;
            return ret;
        },
        __wbg_elapsedTime_5997ee938bdcc694: function(arg0) {
            const ret = getObject(arg0).elapsedTime;
            return ret;
        },
        __wbg_entries_04b37a02507f1713: function(arg0) {
            const ret = Object.entries(getObject(arg0));
            return addHeapObject(ret);
        },
        __wbg_entries_206899f1a3dd30a8: function(arg0) {
            const ret = getObject(arg0).entries();
            return addHeapObject(ret);
        },
        __wbg_error_78ff5b3a29b770e0: function(arg0) {
            console.error(getObject(arg0));
        },
        __wbg_error_9ad1450feb5d541d: function(arg0, arg1, arg2, arg3) {
            console.error(getObject(arg0), getObject(arg1), getObject(arg2), getObject(arg3));
        },
        __wbg_error_f48cb636668f83b3: function(arg0, arg1) {
            console.error(getObject(arg0), getObject(arg1));
        },
        __wbg_eval_35600f795897d127: function() { return handleError(function (arg0, arg1) {
            const ret = eval(getStringFromWasm0(arg0, arg1));
            return addHeapObject(ret);
        }, arguments); },
        __wbg_fetch_8d9b732df7467c44: function(arg0) {
            const ret = fetch(getObject(arg0));
            return addHeapObject(ret);
        },
        __wbg_files_6d12882157f4016a: function(arg0) {
            const ret = getObject(arg0).files;
            return isLikeNone(ret) ? 0 : addHeapObject(ret);
        },
        __wbg_files_e558387d1120f356: function(arg0) {
            const ret = getObject(arg0).files;
            return isLikeNone(ret) ? 0 : addHeapObject(ret);
        },
        __wbg_focus_a3a6c806c5bccea1: function() { return handleError(function (arg0) {
            getObject(arg0).focus();
        }, arguments); },
        __wbg_force_df887cff6d21dd3d: function(arg0) {
            const ret = getObject(arg0).force;
            return ret;
        },
        __wbg_forward_eeed37582baadfaf: function() { return handleError(function (arg0) {
            getObject(arg0).forward();
        }, arguments); },
        __wbg_getAttribute_27f7a0d74339fd41: function(arg0, arg1, arg2, arg3) {
            const ret = getObject(arg1).getAttribute(getStringFromWasm0(arg2, arg3));
            var ptr1 = isLikeNone(ret) ? 0 : passStringToWasm0(ret, wasm.__wbindgen_export, wasm.__wbindgen_export2);
            var len1 = WASM_VECTOR_LEN;
            getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
            getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
        },
        __wbg_getBoundingClientRect_93c2750834277567: function(arg0) {
            const ret = getObject(arg0).getBoundingClientRect();
            return addHeapObject(ret);
        },
        __wbg_getElementById_c35b4b7d270d161d: function(arg0, arg1, arg2) {
            const ret = getObject(arg0).getElementById(getStringFromWasm0(arg1, arg2));
            return isLikeNone(ret) ? 0 : addHeapObject(ret);
        },
        __wbg_getItem_4f09713b84e02011: function() { return handleError(function (arg0, arg1, arg2, arg3) {
            const ret = getObject(arg1).getItem(getStringFromWasm0(arg2, arg3));
            var ptr1 = isLikeNone(ret) ? 0 : passStringToWasm0(ret, wasm.__wbindgen_export, wasm.__wbindgen_export2);
            var len1 = WASM_VECTOR_LEN;
            getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
            getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
        }, arguments); },
        __wbg_getNode_e4cbd5b2e3a6ab5a: function(arg0, arg1) {
            const ret = getObject(arg0).getNode(arg1 >>> 0);
            return addHeapObject(ret);
        },
        __wbg_getRandomValues_ef12552bf5acd2fe: function() { return handleError(function (arg0, arg1) {
            globalThis.crypto.getRandomValues(getArrayU8FromWasm0(arg0, arg1));
        }, arguments); },
        __wbg_getTime_00b3f7db575e4ef5: function(arg0) {
            const ret = getObject(arg0).getTime();
            return ret;
        },
        __wbg_getTimezoneOffset_08e2892156231088: function(arg0) {
            const ret = getObject(arg0).getTimezoneOffset();
            return ret;
        },
        __wbg_get_1f8f054ddbaa7db2: function() { return handleError(function (arg0, arg1) {
            const ret = Reflect.get(getObject(arg0), getObject(arg1));
            return addHeapObject(ret);
        }, arguments); },
        __wbg_get_2b48c7d0d006a781: function(arg0, arg1) {
            const ret = getObject(arg0)[arg1 >>> 0];
            return addHeapObject(ret);
        },
        __wbg_get_c4eb9c8665471988: function(arg0, arg1) {
            const ret = getObject(arg0)[arg1 >>> 0];
            return isLikeNone(ret) ? 0 : addHeapObject(ret);
        },
        __wbg_get_de6a0f7d4d18a304: function() { return handleError(function (arg0, arg1) {
            const ret = Reflect.get(getObject(arg0), getObject(arg1));
            return addHeapObject(ret);
        }, arguments); },
        __wbg_get_form_data_f49e47ffd4157bf2: function(arg0) {
            const ret = get_form_data(getObject(arg0));
            return addHeapObject(ret);
        },
        __wbg_get_select_data_473a79351dba5d30: function(arg0, arg1) {
            const ret = get_select_data(getObject(arg1));
            const ptr1 = passArrayJsValueToWasm0(ret, wasm.__wbindgen_export);
            const len1 = WASM_VECTOR_LEN;
            getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
            getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
        },
        __wbg_get_unchecked_33f6e5c9e2f2d6b2: function(arg0, arg1) {
            const ret = getObject(arg0)[arg1 >>> 0];
            return addHeapObject(ret);
        },
        __wbg_hash_943e3a1af6d171d2: function() { return handleError(function (arg0, arg1) {
            const ret = getObject(arg1).hash;
            const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_export, wasm.__wbindgen_export2);
            const len1 = WASM_VECTOR_LEN;
            getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
            getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
        }, arguments); },
        __wbg_height_87fd6bb4af652876: function(arg0) {
            const ret = getObject(arg0).height;
            return ret;
        },
        __wbg_height_8e3b6ac1a60655fb: function(arg0) {
            const ret = getObject(arg0).height;
            return ret;
        },
        __wbg_height_8e6603d2bea90dc1: function(arg0) {
            const ret = getObject(arg0).height;
            return ret;
        },
        __wbg_history_58333fc71c2a7689: function() { return handleError(function (arg0) {
            const ret = getObject(arg0).history;
            return addHeapObject(ret);
        }, arguments); },
        __wbg_identifier_ea7042cbf4a863b9: function(arg0) {
            const ret = getObject(arg0).identifier;
            return ret;
        },
        __wbg_initialize_a21339c1084649a0: function(arg0, arg1, arg2) {
            getObject(arg0).initialize(takeObject(arg1), getObject(arg2));
        },
        __wbg_inlineSize_370a3eb7b97dc084: function(arg0) {
            const ret = getObject(arg0).inlineSize;
            return ret;
        },
        __wbg_instanceof_ArrayBuffer_8f49811467741499: function(arg0) {
            let result;
            try {
                result = getObject(arg0) instanceof ArrayBuffer;
            } catch (_) {
                result = false;
            }
            const ret = result;
            return ret;
        },
        __wbg_instanceof_CustomEvent_c9861695009b62e1: function(arg0) {
            let result;
            try {
                result = getObject(arg0) instanceof CustomEvent;
            } catch (_) {
                result = false;
            }
            const ret = result;
            return ret;
        },
        __wbg_instanceof_DragEvent_712f9ab18208a9df: function(arg0) {
            let result;
            try {
                result = getObject(arg0) instanceof DragEvent;
            } catch (_) {
                result = false;
            }
            const ret = result;
            return ret;
        },
        __wbg_instanceof_Element_73566cb5986eac3a: function(arg0) {
            let result;
            try {
                result = getObject(arg0) instanceof Element;
            } catch (_) {
                result = false;
            }
            const ret = result;
            return ret;
        },
        __wbg_instanceof_Error_94c8c9d9e410014a: function(arg0) {
            let result;
            try {
                result = getObject(arg0) instanceof Error;
            } catch (_) {
                result = false;
            }
            const ret = result;
            return ret;
        },
        __wbg_instanceof_HtmlElement_9d326f7a42217802: function(arg0) {
            let result;
            try {
                result = getObject(arg0) instanceof HTMLElement;
            } catch (_) {
                result = false;
            }
            const ret = result;
            return ret;
        },
        __wbg_instanceof_HtmlFormElement_fb5be6b5f88d4b6d: function(arg0) {
            let result;
            try {
                result = getObject(arg0) instanceof HTMLFormElement;
            } catch (_) {
                result = false;
            }
            const ret = result;
            return ret;
        },
        __wbg_instanceof_HtmlInputElement_684a7e5d7dbec24c: function(arg0) {
            let result;
            try {
                result = getObject(arg0) instanceof HTMLInputElement;
            } catch (_) {
                result = false;
            }
            const ret = result;
            return ret;
        },
        __wbg_instanceof_HtmlSelectElement_9678e03521a565e0: function(arg0) {
            let result;
            try {
                result = getObject(arg0) instanceof HTMLSelectElement;
            } catch (_) {
                result = false;
            }
            const ret = result;
            return ret;
        },
        __wbg_instanceof_HtmlTextAreaElement_ddb0b111033391f4: function(arg0) {
            let result;
            try {
                result = getObject(arg0) instanceof HTMLTextAreaElement;
            } catch (_) {
                result = false;
            }
            const ret = result;
            return ret;
        },
        __wbg_instanceof_Node_3149777213c50c67: function(arg0) {
            let result;
            try {
                result = getObject(arg0) instanceof Node;
            } catch (_) {
                result = false;
            }
            const ret = result;
            return ret;
        },
        __wbg_instanceof_Promise_d0db99486956c8e8: function(arg0) {
            let result;
            try {
                result = getObject(arg0) instanceof Promise;
            } catch (_) {
                result = false;
            }
            const ret = result;
            return ret;
        },
        __wbg_instanceof_Response_cb984bd66d7bd408: function(arg0) {
            let result;
            try {
                result = getObject(arg0) instanceof Response;
            } catch (_) {
                result = false;
            }
            const ret = result;
            return ret;
        },
        __wbg_instanceof_Uint8Array_86f30649f63ef9c2: function(arg0) {
            let result;
            try {
                result = getObject(arg0) instanceof Uint8Array;
            } catch (_) {
                result = false;
            }
            const ret = result;
            return ret;
        },
        __wbg_instanceof_Window_e093be59ee9a8e14: function(arg0) {
            let result;
            try {
                result = getObject(arg0) instanceof Window;
            } catch (_) {
                result = false;
            }
            const ret = result;
            return ret;
        },
        __wbg_intersectionRatio_00cb91bc55bb82a0: function(arg0) {
            const ret = getObject(arg0).intersectionRatio;
            return ret;
        },
        __wbg_intersectionRect_a76f50fe0931a3a7: function(arg0) {
            const ret = getObject(arg0).intersectionRect;
            return addHeapObject(ret);
        },
        __wbg_isArray_67c2c9c4313f4448: function(arg0) {
            const ret = Array.isArray(getObject(arg0));
            return ret;
        },
        __wbg_isComposing_0b9f69114b4de822: function(arg0) {
            const ret = getObject(arg0).isComposing;
            return ret;
        },
        __wbg_isIntersecting_702e9334ce71bbb4: function(arg0) {
            const ret = getObject(arg0).isIntersecting;
            return ret;
        },
        __wbg_isPrimary_cd1e73d5446d0d3c: function(arg0) {
            const ret = getObject(arg0).isPrimary;
            return ret;
        },
        __wbg_isSafeInteger_66acec27e09e99a7: function(arg0) {
            const ret = Number.isSafeInteger(getObject(arg0));
            return ret;
        },
        __wbg_item_869a4237f79e82fe: function(arg0, arg1) {
            const ret = getObject(arg0).item(arg1 >>> 0);
            return isLikeNone(ret) ? 0 : addHeapObject(ret);
        },
        __wbg_iterator_8732428d309e270e: function() {
            const ret = Symbol.iterator;
            return addHeapObject(ret);
        },
        __wbg_key_df6a54e3e036c3fe: function(arg0, arg1) {
            const ret = getObject(arg1).key;
            const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_export, wasm.__wbindgen_export2);
            const len1 = WASM_VECTOR_LEN;
            getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
            getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
        },
        __wbg_left_985fa07b897d8e74: function(arg0) {
            const ret = getObject(arg0).left;
            return ret;
        },
        __wbg_length_4a591ecaa01354d9: function(arg0) {
            const ret = getObject(arg0).length;
            return ret;
        },
        __wbg_length_521edf73d0cf740d: function(arg0) {
            const ret = getObject(arg0).length;
            return ret;
        },
        __wbg_length_60b522d4443ec08e: function(arg0) {
            const ret = getObject(arg0).length;
            return ret;
        },
        __wbg_length_66f1a4b2e9026940: function(arg0) {
            const ret = getObject(arg0).length;
            return ret;
        },
        __wbg_length_e6080b6174ddb9c0: function(arg0) {
            const ret = getObject(arg0).length;
            return ret;
        },
        __wbg_localStorage_a1773c6a8a951c81: function() { return handleError(function (arg0) {
            const ret = getObject(arg0).localStorage;
            return isLikeNone(ret) ? 0 : addHeapObject(ret);
        }, arguments); },
        __wbg_location_bd39406d76c6d592: function(arg0) {
            const ret = getObject(arg0).location;
            return ret;
        },
        __wbg_location_efdf1fea18b5552a: function(arg0) {
            const ret = getObject(arg0).location;
            return addHeapObject(ret);
        },
        __wbg_log_cf2e968649f3384e: function(arg0) {
            console.log(getObject(arg0));
        },
        __wbg_message_40300ed2d1f8bdc6: function(arg0) {
            const ret = getObject(arg0).message;
            return addHeapObject(ret);
        },
        __wbg_metaKey_752862905c708ca9: function(arg0) {
            const ret = getObject(arg0).metaKey;
            return ret;
        },
        __wbg_metaKey_d2a47aa621ff2c45: function(arg0) {
            const ret = getObject(arg0).metaKey;
            return ret;
        },
        __wbg_metaKey_d3d574e41436dc65: function(arg0) {
            const ret = getObject(arg0).metaKey;
            return ret;
        },
        __wbg_name_2b1afe861fc08636: function(arg0, arg1) {
            const ret = getObject(arg1).name;
            const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_export, wasm.__wbindgen_export2);
            const len1 = WASM_VECTOR_LEN;
            getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
            getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
        },
        __wbg_name_2e3d97e5d7abee6d: function(arg0) {
            const ret = getObject(arg0).name;
            return addHeapObject(ret);
        },
        __wbg_navigator_3833ecdbc19d2757: function(arg0) {
            const ret = getObject(arg0).navigator;
            return addHeapObject(ret);
        },
        __wbg_new_0_445c13a750296eb6: function() {
            const ret = new Date();
            return addHeapObject(ret);
        },
        __wbg_new_372ce3c2fa948cf0: function(arg0) {
            const ret = new RawInterpreter(arg0 >>> 0);
            return addHeapObject(ret);
        },
        __wbg_new_50991c2a032c9857: function() { return handleError(function () {
            const ret = new URLSearchParams();
            return addHeapObject(ret);
        }, arguments); },
        __wbg_new_578aeef4b6b94378: function(arg0) {
            const ret = new Uint8Array(getObject(arg0));
            return addHeapObject(ret);
        },
        __wbg_new_6114450be2525299: function() { return handleError(function () {
            const ret = new FileReader();
            return addHeapObject(ret);
        }, arguments); },
        __wbg_new_622fc80556be2e26: function() {
            const ret = new Map();
            return addHeapObject(ret);
        },
        __wbg_new_6d75fd236f920a62: function(arg0) {
            const ret = new Date(getObject(arg0));
            return addHeapObject(ret);
        },
        __wbg_new_8fa6587d8f37e50d: function(arg0) {
            const ret = new WebDioxusChannel(JSOwner.__wrap(arg0));
            return addHeapObject(ret);
        },
        __wbg_new_b733f187b7a8b317: function() { return handleError(function (arg0, arg1) {
            const ret = new URL(getStringFromWasm0(arg0, arg1));
            return addHeapObject(ret);
        }, arguments); },
        __wbg_new_ce1ab61c1c2b300d: function() {
            const ret = new Object();
            return addHeapObject(ret);
        },
        __wbg_new_d90091b82fdf5b91: function() {
            const ret = new Array();
            return addHeapObject(ret);
        },
        __wbg_new_e436d06bc8e77460: function() { return handleError(function () {
            const ret = new Headers();
            return addHeapObject(ret);
        }, arguments); },
        __wbg_new_with_args_b7569746028c197c: function(arg0, arg1, arg2, arg3) {
            const ret = new Function(getStringFromWasm0(arg0, arg1), getStringFromWasm0(arg2, arg3));
            return addHeapObject(ret);
        },
        __wbg_new_with_str_7a7ea7a290f52724: function() { return handleError(function (arg0, arg1) {
            const ret = new Request(getStringFromWasm0(arg0, arg1));
            return addHeapObject(ret);
        }, arguments); },
        __wbg_new_with_str_and_init_bcd02b79a793d27f: function() { return handleError(function (arg0, arg1, arg2) {
            const ret = new Request(getStringFromWasm0(arg0, arg1), getObject(arg2));
            return addHeapObject(ret);
        }, arguments); },
        __wbg_new_with_str_sequence_eabf85a6027f8970: function() { return handleError(function (arg0) {
            const ret = new Blob(getObject(arg0));
            return addHeapObject(ret);
        }, arguments); },
        __wbg_next_9e03acdf51c4960d: function(arg0) {
            const ret = getObject(arg0).next;
            return addHeapObject(ret);
        },
        __wbg_next_eb8ca7351fa27906: function() { return handleError(function (arg0) {
            const ret = getObject(arg0).next();
            return addHeapObject(ret);
        }, arguments); },
        __wbg_now_190933fa139cc119: function() {
            const ret = Date.now();
            return ret;
        },
        __wbg_offsetX_9ed7cd6abcf67d8b: function(arg0) {
            const ret = getObject(arg0).offsetX;
            return ret;
        },
        __wbg_offsetY_31fc6016c060dacd: function(arg0) {
            const ret = getObject(arg0).offsetY;
            return ret;
        },
        __wbg_ok_fb13c30bc1893039: function(arg0) {
            const ret = getObject(arg0).ok;
            return ret;
        },
        __wbg_open_feb75fe4d9971c50: function() { return handleError(function (arg0, arg1, arg2, arg3, arg4) {
            const ret = getObject(arg0).open(getStringFromWasm0(arg1, arg2), getStringFromWasm0(arg3, arg4));
            return isLikeNone(ret) ? 0 : addHeapObject(ret);
        }, arguments); },
        __wbg_origin_d4fcb992a8589439: function() { return handleError(function (arg0, arg1) {
            const ret = getObject(arg1).origin;
            const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_export, wasm.__wbindgen_export2);
            const len1 = WASM_VECTOR_LEN;
            getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
            getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
        }, arguments); },
        __wbg_ownerDocument_3382cef744616ba1: function(arg0) {
            const ret = getObject(arg0).ownerDocument;
            return isLikeNone(ret) ? 0 : addHeapObject(ret);
        },
        __wbg_pageX_a10e815efab68f0b: function(arg0) {
            const ret = getObject(arg0).pageX;
            return ret;
        },
        __wbg_pageX_e6c22dee653091ec: function(arg0) {
            const ret = getObject(arg0).pageX;
            return ret;
        },
        __wbg_pageY_d267a50f1054ace5: function(arg0) {
            const ret = getObject(arg0).pageY;
            return ret;
        },
        __wbg_pageY_dead0e5826c04c0e: function(arg0) {
            const ret = getObject(arg0).pageY;
            return ret;
        },
        __wbg_parentElement_8b5d8fe9f2319ca8: function(arg0) {
            const ret = getObject(arg0).parentElement;
            return isLikeNone(ret) ? 0 : addHeapObject(ret);
        },
        __wbg_pathname_626d5a44ea16c0c9: function() { return handleError(function (arg0, arg1) {
            const ret = getObject(arg1).pathname;
            const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_export, wasm.__wbindgen_export2);
            const len1 = WASM_VECTOR_LEN;
            getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
            getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
        }, arguments); },
        __wbg_pointerId_be45f553bf33ecdd: function(arg0) {
            const ret = getObject(arg0).pointerId;
            return ret;
        },
        __wbg_pointerType_2da062eba6f661c0: function(arg0, arg1) {
            const ret = getObject(arg1).pointerType;
            const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_export, wasm.__wbindgen_export2);
            const len1 = WASM_VECTOR_LEN;
            getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
            getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
        },
        __wbg_pressure_e31046ec6e6263d3: function(arg0) {
            const ret = getObject(arg0).pressure;
            return ret;
        },
        __wbg_preventDefault_4902f41a1b31bedd: function(arg0) {
            getObject(arg0).preventDefault();
        },
        __wbg_propertyName_1e8546936b567c8b: function(arg0, arg1) {
            const ret = getObject(arg1).propertyName;
            const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_export, wasm.__wbindgen_export2);
            const len1 = WASM_VECTOR_LEN;
            getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
            getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
        },
        __wbg_prototypesetcall_3249fc62a0fafa30: function(arg0, arg1, arg2) {
            Uint8Array.prototype.set.call(getArrayU8FromWasm0(arg0, arg1), getObject(arg2));
        },
        __wbg_pseudoElement_56c2ea08c5ae9a25: function(arg0, arg1) {
            const ret = getObject(arg1).pseudoElement;
            const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_export, wasm.__wbindgen_export2);
            const len1 = WASM_VECTOR_LEN;
            getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
            getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
        },
        __wbg_pseudoElement_907514ceb48d2efa: function(arg0, arg1) {
            const ret = getObject(arg1).pseudoElement;
            const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_export, wasm.__wbindgen_export2);
            const len1 = WASM_VECTOR_LEN;
            getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
            getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
        },
        __wbg_pushState_f4bbfda83cdfe6c2: function() { return handleError(function (arg0, arg1, arg2, arg3, arg4, arg5) {
            getObject(arg0).pushState(getObject(arg1), getStringFromWasm0(arg2, arg3), arg4 === 0 ? undefined : getStringFromWasm0(arg4, arg5));
        }, arguments); },
        __wbg_push_a6822215aa43e71c: function(arg0, arg1) {
            const ret = getObject(arg0).push(getObject(arg1));
            return ret;
        },
        __wbg_queueMicrotask_35c611f4a14830b2: function(arg0) {
            queueMicrotask(getObject(arg0));
        },
        __wbg_queueMicrotask_404ed0a58e0b63cc: function(arg0) {
            const ret = getObject(arg0).queueMicrotask;
            return addHeapObject(ret);
        },
        __wbg_radiusX_ca7ad662bbfb4f57: function(arg0) {
            const ret = getObject(arg0).radiusX;
            return ret;
        },
        __wbg_radiusY_6cdedb371f9c1bd5: function(arg0) {
            const ret = getObject(arg0).radiusY;
            return ret;
        },
        __wbg_readAsArrayBuffer_dcc71a4ed6cdcf06: function() { return handleError(function (arg0, arg1) {
            getObject(arg0).readAsArrayBuffer(getObject(arg1));
        }, arguments); },
        __wbg_readAsText_1892e1cf58cb4930: function() { return handleError(function (arg0, arg1) {
            getObject(arg0).readAsText(getObject(arg1));
        }, arguments); },
        __wbg_removeEventListener_9e2e49dbe3ca4858: function() { return handleError(function (arg0, arg1, arg2, arg3, arg4) {
            getObject(arg0).removeEventListener(getStringFromWasm0(arg1, arg2), getObject(arg3), arg4 !== 0);
        }, arguments); },
        __wbg_removeItem_ebb663d82d1e290a: function() { return handleError(function (arg0, arg1, arg2) {
            getObject(arg0).removeItem(getStringFromWasm0(arg1, arg2));
        }, arguments); },
        __wbg_repeat_136a627581271bd3: function(arg0) {
            const ret = getObject(arg0).repeat;
            return ret;
        },
        __wbg_replaceState_827056d3e0653693: function() { return handleError(function (arg0, arg1, arg2, arg3, arg4, arg5) {
            getObject(arg0).replaceState(getObject(arg1), getStringFromWasm0(arg2, arg3), arg4 === 0 ? undefined : getStringFromWasm0(arg4, arg5));
        }, arguments); },
        __wbg_requestAnimationFrame_72bbc2f340fc7a29: function() { return handleError(function (arg0, arg1) {
            const ret = getObject(arg0).requestAnimationFrame(getObject(arg1));
            return ret;
        }, arguments); },
        __wbg_resolve_25a7e548d5881dca: function(arg0) {
            const ret = Promise.resolve(getObject(arg0));
            return addHeapObject(ret);
        },
        __wbg_result_2804124b89ae22a3: function() { return handleError(function (arg0) {
            const ret = getObject(arg0).result;
            return addHeapObject(ret);
        }, arguments); },
        __wbg_revokeObjectURL_02f29532cbc52b60: function() { return handleError(function (arg0, arg1) {
            URL.revokeObjectURL(getStringFromWasm0(arg0, arg1));
        }, arguments); },
        __wbg_rootBounds_3e53fd34eea77f95: function(arg0) {
            const ret = getObject(arg0).rootBounds;
            return isLikeNone(ret) ? 0 : addHeapObject(ret);
        },
        __wbg_rotationAngle_7b2ec4a2283ff78c: function(arg0) {
            const ret = getObject(arg0).rotationAngle;
            return ret;
        },
        __wbg_run_94b653013f67b6ee: function(arg0) {
            getObject(arg0).run();
        },
        __wbg_rustRecv_8dcea8222613fb2d: function(arg0) {
            const ret = getObject(arg0).rustRecv();
            return addHeapObject(ret);
        },
        __wbg_rustSend_9adbb7c626035edb: function(arg0, arg1) {
            getObject(arg0).rustSend(takeObject(arg1));
        },
        __wbg_saveTemplate_fac2519b3ae9067e: function(arg0, arg1, arg2, arg3) {
            var v0 = getArrayJsValueFromWasm0(arg1, arg2).slice();
            wasm.__wbindgen_export4(arg1, arg2 * 4, 4);
            getObject(arg0).saveTemplate(v0, arg3);
        },
        __wbg_screenX_cf6973d471eef9af: function(arg0) {
            const ret = getObject(arg0).screenX;
            return ret;
        },
        __wbg_screenX_d72df4af68a5e385: function(arg0) {
            const ret = getObject(arg0).screenX;
            return ret;
        },
        __wbg_screenY_aa5662ddfe8bd0b3: function(arg0) {
            const ret = getObject(arg0).screenY;
            return ret;
        },
        __wbg_screenY_e0d554d8c6125f4a: function(arg0) {
            const ret = getObject(arg0).screenY;
            return ret;
        },
        __wbg_scrollHeight_28eca057d00b2962: function(arg0) {
            const ret = getObject(arg0).scrollHeight;
            return ret;
        },
        __wbg_scrollIntoView_26d2401245f96cab: function(arg0, arg1) {
            getObject(arg0).scrollIntoView(getObject(arg1));
        },
        __wbg_scrollLeft_7897f8332705d017: function(arg0) {
            const ret = getObject(arg0).scrollLeft;
            return ret;
        },
        __wbg_scrollTo_76d88a4c0481b97b: function(arg0, arg1, arg2) {
            getObject(arg0).scrollTo(arg1, arg2);
        },
        __wbg_scrollTop_705ef336e30f2d3c: function(arg0) {
            const ret = getObject(arg0).scrollTop;
            return ret;
        },
        __wbg_scrollWidth_48bec0be1eaedf88: function(arg0) {
            const ret = getObject(arg0).scrollWidth;
            return ret;
        },
        __wbg_scrollX_9c3fa310cb5ed346: function() { return handleError(function (arg0) {
            const ret = getObject(arg0).scrollX;
            return ret;
        }, arguments); },
        __wbg_scrollY_5502bd75aa44fd39: function() { return handleError(function (arg0) {
            const ret = getObject(arg0).scrollY;
            return ret;
        }, arguments); },
        __wbg_search_8b96c939223a26e5: function(arg0, arg1) {
            const ret = getObject(arg1).search;
            const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_export, wasm.__wbindgen_export2);
            const len1 = WASM_VECTOR_LEN;
            getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
            getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
        },
        __wbg_search_c299eeae1d9b169e: function() { return handleError(function (arg0, arg1) {
            const ret = getObject(arg1).search;
            const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_export, wasm.__wbindgen_export2);
            const len1 = WASM_VECTOR_LEN;
            getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
            getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
        }, arguments); },
        __wbg_sessionStorage_0a8b0f4b6c397d58: function() { return handleError(function (arg0) {
            const ret = getObject(arg0).sessionStorage;
            return isLikeNone(ret) ? 0 : addHeapObject(ret);
        }, arguments); },
        __wbg_setAttributeInner_f7b6604f435d2990: function(arg0, arg1, arg2, arg3, arg4, arg5) {
            setAttributeInner(takeObject(arg0), getStringFromWasm0(arg1, arg2), takeObject(arg3), arg4 === 0 ? undefined : getStringFromWasm0(arg4, arg5));
        },
        __wbg_setAttribute_5b695d1c3be2e3e6: function() { return handleError(function (arg0, arg1, arg2, arg3, arg4) {
            getObject(arg0).setAttribute(getStringFromWasm0(arg1, arg2), getStringFromWasm0(arg3, arg4));
        }, arguments); },
        __wbg_setItem_bb2b8d07227815d5: function() { return handleError(function (arg0, arg1, arg2, arg3, arg4) {
            getObject(arg0).setItem(getStringFromWasm0(arg1, arg2), getStringFromWasm0(arg3, arg4));
        }, arguments); },
        __wbg_setTimeout_ef24d2fc3ad97385: function() { return handleError(function (arg0, arg1) {
            const ret = setTimeout(getObject(arg0), arg1);
            return addHeapObject(ret);
        }, arguments); },
        __wbg_set_25ef40a9aeff260d: function() { return handleError(function (arg0, arg1, arg2, arg3, arg4) {
            getObject(arg0).set(getStringFromWasm0(arg1, arg2), getStringFromWasm0(arg3, arg4));
        }, arguments); },
        __wbg_set_52b1e1eb5bed906a: function(arg0, arg1, arg2) {
            const ret = getObject(arg0).set(getObject(arg1), getObject(arg2));
            return addHeapObject(ret);
        },
        __wbg_set_6e30c9374c26414c: function() { return handleError(function (arg0, arg1, arg2) {
            const ret = Reflect.set(getObject(arg0), getObject(arg1), getObject(arg2));
            return ret;
        }, arguments); },
        __wbg_set_behavior_0ee56867a7179068: function(arg0, arg1) {
            getObject(arg0).behavior = __wbindgen_enum_ScrollBehavior[arg1];
        },
        __wbg_set_body_36614c7e61546809: function(arg0, arg1) {
            getObject(arg0).body = getObject(arg1);
        },
        __wbg_set_capture_470b54e0a5c98d83: function(arg0, arg1) {
            getObject(arg0).capture = arg1 !== 0;
        },
        __wbg_set_dca99999bba88a9a: function(arg0, arg1, arg2) {
            getObject(arg0)[arg1 >>> 0] = takeObject(arg2);
        },
        __wbg_set_f071dbb3bd088e0e: function(arg0, arg1, arg2) {
            getObject(arg0)[takeObject(arg1)] = takeObject(arg2);
        },
        __wbg_set_headers_7c1e39ece7826bec: function(arg0, arg1) {
            getObject(arg0).headers = getObject(arg1);
        },
        __wbg_set_href_eee50816257e9d44: function() { return handleError(function (arg0, arg1, arg2) {
            getObject(arg0).href = getStringFromWasm0(arg1, arg2);
        }, arguments); },
        __wbg_set_innerHTML_6bcbbce0a3626998: function(arg0, arg1, arg2) {
            getObject(arg0).innerHTML = getStringFromWasm0(arg1, arg2);
        },
        __wbg_set_method_7a6811dec7a4feff: function(arg0, arg1, arg2) {
            getObject(arg0).method = getStringFromWasm0(arg1, arg2);
        },
        __wbg_set_once_27df5233613a51cd: function(arg0, arg1) {
            getObject(arg0).once = arg1 !== 0;
        },
        __wbg_set_onload_c8366a4de7e51593: function(arg0, arg1) {
            getObject(arg0).onload = getObject(arg1);
        },
        __wbg_set_passive_cd1f03dae3bdd0f9: function(arg0, arg1) {
            getObject(arg0).passive = arg1 !== 0;
        },
        __wbg_set_scrollRestoration_2dbe7bb81342e294: function() { return handleError(function (arg0, arg1) {
            getObject(arg0).scrollRestoration = __wbindgen_enum_ScrollRestoration[arg1];
        }, arguments); },
        __wbg_set_search_c12c3710c7e7761e: function(arg0, arg1, arg2) {
            getObject(arg0).search = getStringFromWasm0(arg1, arg2);
        },
        __wbg_shiftKey_05941b44ffe0a9ce: function(arg0) {
            const ret = getObject(arg0).shiftKey;
            return ret;
        },
        __wbg_shiftKey_e62ee708439d8a9c: function(arg0) {
            const ret = getObject(arg0).shiftKey;
            return ret;
        },
        __wbg_shiftKey_ec95aec36c86fb31: function(arg0) {
            const ret = getObject(arg0).shiftKey;
            return ret;
        },
        __wbg_size_9970092b88b1094c: function(arg0) {
            const ret = getObject(arg0).size;
            return ret;
        },
        __wbg_state_7824bbeefbc301b1: function() { return handleError(function (arg0) {
            const ret = getObject(arg0).state;
            return addHeapObject(ret);
        }, arguments); },
        __wbg_static_accessor_GLOBAL_9d53f2689e622ca1: function() {
            const ret = typeof global === 'undefined' ? null : global;
            return isLikeNone(ret) ? 0 : addHeapObject(ret);
        },
        __wbg_static_accessor_GLOBAL_THIS_a1a35cec07001a8a: function() {
            const ret = typeof globalThis === 'undefined' ? null : globalThis;
            return isLikeNone(ret) ? 0 : addHeapObject(ret);
        },
        __wbg_static_accessor_SELF_4c59f6c7ea29a144: function() {
            const ret = typeof self === 'undefined' ? null : self;
            return isLikeNone(ret) ? 0 : addHeapObject(ret);
        },
        __wbg_static_accessor_WINDOW_e70ae9f2eb052253: function() {
            const ret = typeof window === 'undefined' ? null : window;
            return isLikeNone(ret) ? 0 : addHeapObject(ret);
        },
        __wbg_status_00549d55b78d949e: function(arg0) {
            const ret = getObject(arg0).status;
            return ret;
        },
        __wbg_stringify_8286df6dcc591521: function() { return handleError(function (arg0) {
            const ret = JSON.stringify(getObject(arg0));
            return addHeapObject(ret);
        }, arguments); },
        __wbg_tangentialPressure_86b5261e93500a5e: function(arg0) {
            const ret = getObject(arg0).tangentialPressure;
            return ret;
        },
        __wbg_targetTouches_775bc5755b8b96d9: function(arg0) {
            const ret = getObject(arg0).targetTouches;
            return addHeapObject(ret);
        },
        __wbg_target_baf3e983dceee053: function(arg0) {
            const ret = getObject(arg0).target;
            return isLikeNone(ret) ? 0 : addHeapObject(ret);
        },
        __wbg_textContent_666b896f07901078: function(arg0, arg1) {
            const ret = getObject(arg1).textContent;
            var ptr1 = isLikeNone(ret) ? 0 : passStringToWasm0(ret, wasm.__wbindgen_export, wasm.__wbindgen_export2);
            var len1 = WASM_VECTOR_LEN;
            getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
            getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
        },
        __wbg_text_a17febec76d36501: function() { return handleError(function (arg0) {
            const ret = getObject(arg0).text();
            return addHeapObject(ret);
        }, arguments); },
        __wbg_then_18f476d590e58992: function(arg0, arg1, arg2) {
            const ret = getObject(arg0).then(getObject(arg1), getObject(arg2));
            return addHeapObject(ret);
        },
        __wbg_then_ac7b025999b52837: function(arg0, arg1) {
            const ret = getObject(arg0).then(getObject(arg1));
            return addHeapObject(ret);
        },
        __wbg_tiltX_8d7111f4d9133ef2: function(arg0) {
            const ret = getObject(arg0).tiltX;
            return ret;
        },
        __wbg_tiltY_47632da38d059bfe: function(arg0) {
            const ret = getObject(arg0).tiltY;
            return ret;
        },
        __wbg_time_d7cc1475e076e12d: function(arg0) {
            const ret = getObject(arg0).time;
            return ret;
        },
        __wbg_toString_1fcc5569307bd3f3: function(arg0) {
            const ret = getObject(arg0).toString();
            return addHeapObject(ret);
        },
        __wbg_toString_ceaa3435e9077b9a: function(arg0) {
            const ret = getObject(arg0).toString();
            return addHeapObject(ret);
        },
        __wbg_top_14d766e5bde56568: function(arg0) {
            const ret = getObject(arg0).top;
            return ret;
        },
        __wbg_touches_65e436ac07e6aa05: function(arg0) {
            const ret = getObject(arg0).touches;
            return addHeapObject(ret);
        },
        __wbg_twist_86e2236ace768bb1: function(arg0) {
            const ret = getObject(arg0).twist;
            return ret;
        },
        __wbg_type_6488255feec94876: function(arg0, arg1) {
            const ret = getObject(arg1).type;
            const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_export, wasm.__wbindgen_export2);
            const len1 = WASM_VECTOR_LEN;
            getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
            getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
        },
        __wbg_type_d2c93d2bc26d9b55: function(arg0, arg1) {
            const ret = getObject(arg1).type;
            const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_export, wasm.__wbindgen_export2);
            const len1 = WASM_VECTOR_LEN;
            getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
            getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
        },
        __wbg_update_memory_e347a7fdedc3f923: function(arg0, arg1) {
            getObject(arg0).update_memory(takeObject(arg1));
        },
        __wbg_url_1b5edc4adbba9990: function(arg0, arg1) {
            const ret = getObject(arg1).url;
            const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_export, wasm.__wbindgen_export2);
            const len1 = WASM_VECTOR_LEN;
            getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
            getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
        },
        __wbg_userAgent_8def8135d886414b: function() { return handleError(function (arg0, arg1) {
            const ret = getObject(arg1).userAgent;
            const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_export, wasm.__wbindgen_export2);
            const len1 = WASM_VECTOR_LEN;
            getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
            getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
        }, arguments); },
        __wbg_value_32493aa1c785ef7c: function(arg0, arg1) {
            const ret = getObject(arg1).value;
            const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_export, wasm.__wbindgen_export2);
            const len1 = WASM_VECTOR_LEN;
            getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
            getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
        },
        __wbg_value_6177c7953f900695: function(arg0, arg1) {
            const ret = getObject(arg1).value;
            const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_export, wasm.__wbindgen_export2);
            const len1 = WASM_VECTOR_LEN;
            getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
            getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
        },
        __wbg_value_ed244ce0aafd9d66: function(arg0, arg1) {
            const ret = getObject(arg1).value;
            const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_export, wasm.__wbindgen_export2);
            const len1 = WASM_VECTOR_LEN;
            getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
            getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
        },
        __wbg_value_f3625092ee4b37f4: function(arg0) {
            const ret = getObject(arg0).value;
            return addHeapObject(ret);
        },
        __wbg_warn_410c3261e3c6d686: function(arg0) {
            console.warn(getObject(arg0));
        },
        __wbg_weak_1df2750a32313fd1: function(arg0) {
            const ret = getObject(arg0).weak();
            return addHeapObject(ret);
        },
        __wbg_width_1e0b74fef17bc28b: function(arg0) {
            const ret = getObject(arg0).width;
            return ret;
        },
        __wbg_width_699320f00b42fce1: function(arg0) {
            const ret = getObject(arg0).width;
            return ret;
        },
        __wbg_width_dd13177d91751e3c: function(arg0) {
            const ret = getObject(arg0).width;
            return ret;
        },
        __wbg_writeText_e35fb5c1924eb2ac: function(arg0, arg1, arg2) {
            const ret = getObject(arg0).writeText(getStringFromWasm0(arg1, arg2));
            return addHeapObject(ret);
        },
        __wbg_x_d04fc448eb267831: function(arg0) {
            const ret = getObject(arg0).x;
            return ret;
        },
        __wbg_y_7a4ac1ce3a336bab: function(arg0) {
            const ret = getObject(arg0).y;
            return ret;
        },
        __wbindgen_cast_0000000000000001: function(arg0, arg1) {
            // Cast intrinsic for `Closure(Closure { owned: true, function: Function { arguments: [Externref], shim_idx: 3265, ret: Result(Unit), inner_ret: Some(Result(Unit)) }, mutable: true }) -> Externref`.
            const ret = makeMutClosure(arg0, arg1, __wasm_bindgen_func_elem_18310);
            return addHeapObject(ret);
        },
        __wbindgen_cast_0000000000000002: function(arg0, arg1) {
            // Cast intrinsic for `Closure(Closure { owned: true, function: Function { arguments: [NamedExternref("Event")], shim_idx: 2657, ret: Unit, inner_ret: Some(Unit) }, mutable: true }) -> Externref`.
            const ret = makeMutClosure(arg0, arg1, __wasm_bindgen_func_elem_14465);
            return addHeapObject(ret);
        },
        __wbindgen_cast_0000000000000003: function(arg0, arg1) {
            // Cast intrinsic for `Closure(Closure { owned: true, function: Function { arguments: [Ref(NamedExternref("Event"))], shim_idx: 2658, ret: Unit, inner_ret: Some(Unit) }, mutable: false }) -> Externref`.
            const ret = makeClosure(arg0, arg1, __wasm_bindgen_func_elem_14466);
            return addHeapObject(ret);
        },
        __wbindgen_cast_0000000000000004: function(arg0, arg1) {
            // Cast intrinsic for `Closure(Closure { owned: true, function: Function { arguments: [Ref(NamedExternref("Event"))], shim_idx: 54, ret: Unit, inner_ret: Some(Unit) }, mutable: true }) -> Externref`.
            const ret = makeMutClosure(arg0, arg1, __wasm_bindgen_func_elem_8873);
            return addHeapObject(ret);
        },
        __wbindgen_cast_0000000000000005: function(arg0, arg1) {
            // Cast intrinsic for `Closure(Closure { owned: true, function: Function { arguments: [], shim_idx: 2636, ret: Unit, inner_ret: Some(Unit) }, mutable: true }) -> Externref`.
            const ret = makeMutClosure(arg0, arg1, __wasm_bindgen_func_elem_14059);
            return addHeapObject(ret);
        },
        __wbindgen_cast_0000000000000006: function(arg0, arg1) {
            // Cast intrinsic for `Closure(Closure { owned: true, function: Function { arguments: [], shim_idx: 2656, ret: Unit, inner_ret: Some(Unit) }, mutable: true }) -> Externref`.
            const ret = makeMutClosure(arg0, arg1, __wasm_bindgen_func_elem_14464);
            return addHeapObject(ret);
        },
        __wbindgen_cast_0000000000000007: function(arg0) {
            // Cast intrinsic for `F64 -> Externref`.
            const ret = arg0;
            return addHeapObject(ret);
        },
        __wbindgen_cast_0000000000000008: function(arg0) {
            // Cast intrinsic for `I64 -> Externref`.
            const ret = arg0;
            return addHeapObject(ret);
        },
        __wbindgen_cast_0000000000000009: function(arg0, arg1) {
            // Cast intrinsic for `Ref(String) -> Externref`.
            const ret = getStringFromWasm0(arg0, arg1);
            return addHeapObject(ret);
        },
        __wbindgen_cast_000000000000000a: function(arg0) {
            // Cast intrinsic for `U64 -> Externref`.
            const ret = BigInt.asUintN(64, arg0);
            return addHeapObject(ret);
        },
        __wbindgen_object_clone_ref: function(arg0) {
            const ret = getObject(arg0);
            return addHeapObject(ret);
        },
        __wbindgen_object_drop_ref: function(arg0) {
            takeObject(arg0);
        },
    };
    return {
        __proto__: null,
        "./woody-weed-bot_bg.js": import0,
    };
}

function __wasm_bindgen_func_elem_14059(arg0, arg1) {
    wasm.__wasm_bindgen_func_elem_14059(arg0, arg1);
}

function __wasm_bindgen_func_elem_14464(arg0, arg1) {
    wasm.__wasm_bindgen_func_elem_14464(arg0, arg1);
}

function __wasm_bindgen_func_elem_14465(arg0, arg1, arg2) {
    wasm.__wasm_bindgen_func_elem_14465(arg0, arg1, addHeapObject(arg2));
}

function __wasm_bindgen_func_elem_14466(arg0, arg1, arg2) {
    try {
        wasm.__wasm_bindgen_func_elem_14466(arg0, arg1, addBorrowedObject(arg2));
    } finally {
        heap[stack_pointer++] = undefined;
    }
}

function __wasm_bindgen_func_elem_8873(arg0, arg1, arg2) {
    try {
        wasm.__wasm_bindgen_func_elem_8873(arg0, arg1, addBorrowedObject(arg2));
    } finally {
        heap[stack_pointer++] = undefined;
    }
}

function __wasm_bindgen_func_elem_18310(arg0, arg1, arg2) {
    try {
        const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
        wasm.__wasm_bindgen_func_elem_18310(retptr, arg0, arg1, addHeapObject(arg2));
        var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
        var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
        if (r1) {
            throw takeObject(r0);
        }
    } finally {
        wasm.__wbindgen_add_to_stack_pointer(16);
    }
}


const __wbindgen_enum_ScrollBehavior = ["auto", "instant", "smooth"];


const __wbindgen_enum_ScrollRestoration = ["auto", "manual"];
const JSOwnerFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_jsowner_free(ptr, 1));

function addHeapObject(obj) {
    if (heap_next === heap.length) heap.push(heap.length + 1);
    const idx = heap_next;
    heap_next = heap[idx];

    heap[idx] = obj;
    return idx;
}

function addBorrowedObject(obj) {
    if (stack_pointer == 1) throw new Error('out of js stack');
    heap[--stack_pointer] = obj;
    return stack_pointer;
}

const CLOSURE_DTORS = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(state => wasm.__wbindgen_export5(state.a, state.b));

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
    if (builtInMatches && builtInMatches.length > 1) {
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

function dropObject(idx) {
    if (idx < 1028) return;
    heap[idx] = heap_next;
    heap_next = idx;
}

function getArrayJsValueFromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    const mem = getDataViewMemory0();
    const result = [];
    for (let i = ptr; i < ptr + 4 * len; i += 4) {
        result.push(takeObject(mem.getUint32(i, true)));
    }
    return result;
}

function getArrayU8FromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    return getUint8ArrayMemory0().subarray(ptr / 1, ptr / 1 + len);
}

let cachedDataViewMemory0 = null;
function getDataViewMemory0() {
    if (cachedDataViewMemory0 === null || cachedDataViewMemory0.buffer.detached === true || (cachedDataViewMemory0.buffer.detached === undefined && cachedDataViewMemory0.buffer !== wasm.memory.buffer)) {
        cachedDataViewMemory0 = new DataView(wasm.memory.buffer);
    }
    return cachedDataViewMemory0;
}

function getStringFromWasm0(ptr, len) {
    return decodeText(ptr >>> 0, len);
}

let cachedUint8ArrayMemory0 = null;
function getUint8ArrayMemory0() {
    if (cachedUint8ArrayMemory0 === null || cachedUint8ArrayMemory0.byteLength === 0) {
        cachedUint8ArrayMemory0 = new Uint8Array(wasm.memory.buffer);
    }
    return cachedUint8ArrayMemory0;
}

function getObject(idx) { return heap[idx]; }

function handleError(f, args) {
    try {
        return f.apply(this, args);
    } catch (e) {
        wasm.__wbindgen_export3(addHeapObject(e));
    }
}

let heap = new Array(1024).fill(undefined);
heap.push(undefined, null, true, false);

let heap_next = heap.length;

function isLikeNone(x) {
    return x === undefined || x === null;
}

function makeClosure(arg0, arg1, f) {
    const state = { a: arg0, b: arg1, cnt: 1 };
    const real = (...args) => {

        // First up with a closure we increment the internal reference
        // count. This ensures that the Rust closure environment won't
        // be deallocated while we're invoking it.
        state.cnt++;
        try {
            return f(state.a, state.b, ...args);
        } finally {
            real._wbg_cb_unref();
        }
    };
    real._wbg_cb_unref = () => {
        if (--state.cnt === 0) {
            wasm.__wbindgen_export5(state.a, state.b);
            state.a = 0;
            CLOSURE_DTORS.unregister(state);
        }
    };
    CLOSURE_DTORS.register(real, state, state);
    return real;
}

function makeMutClosure(arg0, arg1, f) {
    const state = { a: arg0, b: arg1, cnt: 1 };
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
            state.a = a;
            real._wbg_cb_unref();
        }
    };
    real._wbg_cb_unref = () => {
        if (--state.cnt === 0) {
            wasm.__wbindgen_export5(state.a, state.b);
            state.a = 0;
            CLOSURE_DTORS.unregister(state);
        }
    };
    CLOSURE_DTORS.register(real, state, state);
    return real;
}

function passArrayJsValueToWasm0(array, malloc) {
    const ptr = malloc(array.length * 4, 4) >>> 0;
    const mem = getDataViewMemory0();
    for (let i = 0; i < array.length; i++) {
        mem.setUint32(ptr + 4 * i, addHeapObject(array[i]), true);
    }
    WASM_VECTOR_LEN = array.length;
    return ptr;
}

function passStringToWasm0(arg, malloc, realloc) {
    if (realloc === undefined) {
        const buf = cachedTextEncoder.encode(arg);
        const ptr = malloc(buf.length, 1) >>> 0;
        getUint8ArrayMemory0().subarray(ptr, ptr + buf.length).set(buf);
        WASM_VECTOR_LEN = buf.length;
        return ptr;
    }

    let len = arg.length;
    let ptr = malloc(len, 1) >>> 0;

    const mem = getUint8ArrayMemory0();

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
        ptr = realloc(ptr, len, len = offset + arg.length * 3, 1) >>> 0;
        const view = getUint8ArrayMemory0().subarray(ptr + offset, ptr + len);
        const ret = cachedTextEncoder.encodeInto(arg, view);

        offset += ret.written;
        ptr = realloc(ptr, len, offset, 1) >>> 0;
    }

    WASM_VECTOR_LEN = offset;
    return ptr;
}

let stack_pointer = 1024;

function takeObject(idx) {
    const ret = getObject(idx);
    dropObject(idx);
    return ret;
}

let cachedTextDecoder = new TextDecoder('utf-8', { ignoreBOM: true, fatal: true });
cachedTextDecoder.decode();
const MAX_SAFARI_DECODE_BYTES = 2146435072;
let numBytesDecoded = 0;
function decodeText(ptr, len) {
    numBytesDecoded += len;
    if (numBytesDecoded >= MAX_SAFARI_DECODE_BYTES) {
        cachedTextDecoder = new TextDecoder('utf-8', { ignoreBOM: true, fatal: true });
        cachedTextDecoder.decode();
        numBytesDecoded = len;
    }
    return cachedTextDecoder.decode(getUint8ArrayMemory0().subarray(ptr, ptr + len));
}

const cachedTextEncoder = new TextEncoder();

if (!('encodeInto' in cachedTextEncoder)) {
    cachedTextEncoder.encodeInto = function (arg, view) {
        const buf = cachedTextEncoder.encode(arg);
        view.set(buf);
        return {
            read: arg.length,
            written: buf.length
        };
    };
}

let WASM_VECTOR_LEN = 0;

let wasmModule, wasmInstance, wasm;
function __wbg_finalize_init(instance, module) {
    wasmInstance = instance;
    wasm = instance.exports;
    wasmModule = module;
    cachedDataViewMemory0 = null;
    cachedUint8ArrayMemory0 = null;
    wasm.__wbindgen_start();
    return wasm;
}

async function __wbg_load(module, imports) {
    if (typeof Response === 'function' && module instanceof Response) {
        if (typeof WebAssembly.instantiateStreaming === 'function') {
            try {
                return await WebAssembly.instantiateStreaming(module, imports);
            } catch (e) {
                const validResponse = module.ok && expectedResponseType(module.type);

                if (validResponse && module.headers.get('Content-Type') !== 'application/wasm') {
                    console.warn("`WebAssembly.instantiateStreaming` failed because your server does not serve Wasm with `application/wasm` MIME type. Falling back to `WebAssembly.instantiate` which is slower. Original error:\n", e);

                } else { throw e; }
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

    function expectedResponseType(type) {
        switch (type) {
            case 'basic': case 'cors': case 'default': return true;
        }
        return false;
    }
}

function initSync(module) {
    if (wasm !== undefined) return wasm;


    if (module !== undefined) {
        if (Object.getPrototypeOf(module) === Object.prototype) {
            ({module} = module)
        } else {
            console.warn('using deprecated parameters for `initSync()`; pass a single object instead')
        }
    }

    const imports = __wbg_get_imports();
    if (!(module instanceof WebAssembly.Module)) {
        module = new WebAssembly.Module(module);
    }
    const instance = new WebAssembly.Instance(module, imports);
    return __wbg_finalize_init(instance, module);
}

async function __wbg_init(module_or_path) {
    if (wasm !== undefined) return wasm;


    if (module_or_path !== undefined) {
        if (Object.getPrototypeOf(module_or_path) === Object.prototype) {
            ({module_or_path} = module_or_path)
        } else {
            console.warn('using deprecated parameters for the initialization function; pass a single object instead')
        }
    }

    if (module_or_path === undefined) {
        module_or_path = new URL('woody-weed-bot_bg.wasm', import.meta.url);
    }
    const imports = __wbg_get_imports();

    if (typeof module_or_path === 'string' || (typeof Request === 'function' && module_or_path instanceof Request) || (typeof URL === 'function' && module_or_path instanceof URL)) {
        module_or_path = fetch(module_or_path);
    }

    const { instance, module } = await __wbg_load(await module_or_path, imports);

    return __wbg_finalize_init(instance, module);
}

export { initSync, __wbg_init as default };
