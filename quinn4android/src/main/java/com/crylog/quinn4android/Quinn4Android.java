package com.crylog.quinn4android;

public class Quinn4Android {
    public Quinn4Android() {
    }
    static {
        System.loadLibrary("quinn-rs-binding");
    }

    private static native String Client_Insecure(String IP);
    private static native String Server_Insecure(String IP);
}
