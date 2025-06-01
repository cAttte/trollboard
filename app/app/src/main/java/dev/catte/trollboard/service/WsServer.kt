package dev.catte.trollboard.service

import org.java_websocket.WebSocket
import org.java_websocket.drafts.Draft
import org.java_websocket.framing.CloseFrame
import org.java_websocket.handshake.ClientHandshake
import org.java_websocket.handshake.ServerHandshakeBuilder
import org.java_websocket.server.WebSocketServer
import java.net.InetSocketAddress

class WsServer(
    port: Int,
    private val trollword: String,
    private val onOpen: (ClientHandshake) -> Unit,
    private val onClose: () -> Unit
) : WebSocketServer(InetSocketAddress(port)) {
    private var conn: WebSocket? = null

    override fun onWebsocketHandshakeReceivedAsServer(
        conn: WebSocket?,
        draft: Draft?,
        request: ClientHandshake?
    ): ServerHandshakeBuilder? {
        val builder = super.onWebsocketHandshakeReceivedAsServer(conn, draft, request)
        builder.put("X-Trollword", trollword)
        return builder
    }

    fun sendMessage(message: String) {
        this.conn?.send(message)
    }

    override fun onOpen(conn: WebSocket?, handshake: ClientHandshake?) {
        if (this.conn != null) conn?.close(CloseFrame.POLICY_VALIDATION, "TROLLBOARD_BUSY")
        if (conn == null || handshake == null) return
        this.conn = conn

        onOpen(handshake)
    }

    override fun onClose(conn: WebSocket?, code: Int, reason: String?, remote: Boolean) {
        if (conn == this.conn) this.conn = null
        onClose()
    }

    override fun onMessage(conn: WebSocket?, message: String?) {}

    override fun onError(conn: WebSocket?, ex: Exception?) {
        ex?.printStackTrace()
    }

    override fun onStart() {}
}
