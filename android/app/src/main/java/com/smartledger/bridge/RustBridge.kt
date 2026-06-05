package com.smartledger.bridge

object RustBridge {
    
    init {
        System.loadLibrary("smartledger")
    }
    
    external fun getVersion(): String
}
