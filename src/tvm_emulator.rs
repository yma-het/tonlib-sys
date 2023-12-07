extern "C" {
    /**
     * @brief Create TVM emulator
     * @param code_boc Base64 encoded BoC serialized smart contract code cell
     * @param data_boc Base64 encoded BoC serialized smart contract data cell
     * @param vm_log_verbosity Verbosity level of VM log
     * @return Pointer to TVM emulator object
     */
    pub fn tvm_emulator_create(
        code: *const ::std::os::raw::c_char,
        data: *const ::std::os::raw::c_char,
        vm_log_verbosity: u32,
    ) -> *mut ::std::os::raw::c_void;
}

extern "C" {
    /**
     * @brief Set libraries for TVM emulator
     * @param libs_boc Base64 encoded BoC serialized libraries dictionary (HashmapE 256 ^Cell).
     * @return true in case of success, false in case of error
     */
    pub fn tvm_emulator_set_libraries(
        tvm_emulator: *mut ::std::os::raw::c_void,
        libs_boc: *const ::std::os::raw::c_char,
    ) -> bool;
}

extern "C" {
    /**
     * @brief Set c7 parameters
     * @param tvm_emulator Pointer to TVM emulator
     * @param address Adress of smart contract
     * @param unixtime Unix timestamp
     * @param balance Smart contract balance
     * @param rand_seed_hex Random seed as hex string of length 64
     * @param config Base64 encoded BoC serialized Config dictionary (Hashmap 32 ^Cell)
     * @return true in case of success, false in case of error
     */
    pub fn tvm_emulator_set_c7(
        tvm_emulator: *mut ::std::os::raw::c_void,
        address: *const ::std::os::raw::c_char,
        unixtime: u32,
        balance: u64,
        rand_seed_hex: *const ::std::os::raw::c_char,
        config: *const ::std::os::raw::c_char,
    ) -> bool;
}

extern "C" {
    /**
     * @brief Set TVM gas limit
     * @param tvm_emulator Pointer to TVM emulator
     * @param gas_limit Gas limit
     * @return true in case of success, false in case of error
     */
    pub fn tvm_emulator_set_gas_limit(
        tvm_emulator: *mut ::std::os::raw::c_void,
        gas_limit: u64,
    ) -> bool;
}

extern "C" {
    /**
     * @brief Enable or disable TVM debug primitives
     * @param tvm_emulator Pointer to TVM emulator
     * @param debug_enabled Whether debug primitives should be enabled or not
     * @return true in case of success, false in case of error
     */
    pub fn tvm_emulator_set_debug_enabled(
        tvm_emulator: *mut ::std::os::raw::c_void,
        debug_enabled: ::std::os::raw::c_int,
    ) -> bool;

}

extern "C" {
    /**
     * @brief Run get method
     * @param tvm_emulator Pointer to TVM emulator
     * @param method_id Integer method id
     * @param stack_boc Base64 encoded BoC serialized stack (VmStack)
     * @return Json object with error:
     * {
     *   "success": false,
     *   "error": "Error description"
     * }
     * Or success:
     * {
     *   "success": true
     *   "vm_log": "...",
     *   "vm_exit_code": 0,
     *   "stack": "Base64 encoded BoC serialized stack (VmStack)",
     *   "missing_library": null,
     *   "gas_used": 1212
     * }
     */
    pub fn tvm_emulator_run_get_method(
        tvm_emulator: *mut ::std::os::raw::c_void,
        method_id: i32,
        stack_boc: *const ::std::os::raw::c_char,
    ) -> *const ::std::os::raw::c_char;
}

extern "C" {
    /**
     * @brief Send external message
     * @param tvm_emulator Pointer to TVM emulator
     * @param message_body_boc Base64 encoded BoC serialized message body cell.
     * @return Json object with error:
     * {
     *   "success": false,
     *   "error": "Error description"
     * }
     * Or success:
     * {
     *   "success": true,
     *   "new_code": "Base64 boc decoded new code cell",
     *   "new_data": "Base64 boc decoded new data cell",
     *   "accepted": true,
     *   "vm_exit_code": 0,
     *   "vm_log": "...",
     *   "missing_library": null,
     *   "gas_used": 1212,
     *   "actions": "Base64 boc decoded actions cell of type (OutList n)"
     * }
     */
    pub fn tvm_emulator_send_external_message(
        tvm_emulator: *mut ::std::os::raw::c_void,
        message_body_boc: *const ::std::os::raw::c_char,
    ) -> *const ::std::os::raw::c_char;
}

extern "C" {
    /**
     * @brief Send internal message
     * @param tvm_emulator Pointer to TVM emulator
     * @param message_body_boc Base64 encoded BoC serialized message body cell.
     * @param amount Amount of nanograms attached with internal message.
     * @return Json object with error:
     * {
     *   "success": false,
     *   "error": "Error description"
     * }
     * Or success:
     * {
     *   "success": true,
     *   "new_code": "Base64 boc decoded new code cell",
     *   "new_data": "Base64 boc decoded new data cell",
     *   "accepted": true,
     *   "vm_exit_code": 0,
     *   "vm_log": "...",
     *   "missing_library": null,
     *   "gas_used": 1212,
     *   "actions": "Base64 boc decoded actions cell of type (OutList n)"
     * }
     */
    pub fn tvm_emulator_send_internal_message(
        tvm_emulator: *mut ::std::os::raw::c_void,
        message_body_boc: *const ::std::os::raw::c_char,
        amount: u64,
    ) -> *const ::std::os::raw::c_char;
}

extern "C" {
    /**
     * @brief Destroy TVM emulator object
     * @param tvm_emulator Pointer to TVM emulator object
     */
    pub fn tvm_emulator_destroy(tvm_emulator: *mut ::std::os::raw::c_void);
}
