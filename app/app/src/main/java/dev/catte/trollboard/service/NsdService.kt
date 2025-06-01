package dev.catte.trollboard.service

import android.content.Context
import android.net.nsd.NsdManager
import android.net.nsd.NsdServiceInfo
import android.util.Log

class NsdService(private val ctx: Context, private val port: Int) {
    private val name = "Trollboard"
    private val type = "_trollboard._tcp."
    val nsdManager: NsdManager
        get() = (ctx.getSystemService(Context.NSD_SERVICE) as NsdManager)

    private val registrationListener = object : NsdManager.RegistrationListener {
        override fun onServiceRegistered(service: NsdServiceInfo) {
            Log.d("MDNS", "service registered")
        }

        override fun onRegistrationFailed(serviceInfo: NsdServiceInfo, errorCode: Int) {
            Log.e("MDNS", "service failed $errorCode")
        }

        override fun onServiceUnregistered(arg0: NsdServiceInfo) {}
        override fun onUnregistrationFailed(serviceInfo: NsdServiceInfo, errorCode: Int) {}
    }

    fun register() {
        val info = NsdServiceInfo().apply {
            serviceName = name
            serviceType = type
            port = this@NsdService.port
        }

        nsdManager.registerService(info, NsdManager.PROTOCOL_DNS_SD, registrationListener)
    }

    fun unregister() {
        nsdManager.unregisterService(registrationListener)
    }
}

