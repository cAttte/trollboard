package dev.catte.trollboard

import android.os.Bundle
import android.view.MotionEvent
import androidx.activity.ComponentActivity
import androidx.activity.compose.setContent
import androidx.activity.enableEdgeToEdge
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.PaddingValues
import androidx.compose.foundation.layout.WindowInsets
import androidx.compose.foundation.layout.asPaddingValues
import androidx.compose.foundation.layout.aspectRatio
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.layout.safeDrawing
import androidx.compose.foundation.layout.size
import androidx.compose.foundation.lazy.grid.GridCells
import androidx.compose.foundation.lazy.grid.LazyVerticalGrid
import androidx.compose.foundation.lazy.grid.items
import androidx.compose.material3.Button
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.Surface
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.runtime.mutableStateListOf
import androidx.compose.ui.Modifier
import androidx.compose.ui.input.pointer.pointerInteropFilter
import androidx.compose.ui.text.style.TextAlign
import androidx.compose.ui.unit.dp
import dev.catte.trollboard.service.NsdService
import dev.catte.trollboard.service.WsServer
import dev.catte.trollboard.ui.component.SvgImage
import dev.catte.trollboard.ui.theme.ComicSans
import dev.catte.trollboard.ui.theme.TrollboardTheme
import kotlinx.serialization.Serializable
import kotlinx.serialization.json.Json
import java.net.ServerSocket

@Serializable
class ButtonInfo(val name: String, val desc: String, val icon: String)

@Serializable
@Suppress("unused")
class ButtonAction(val button: String, val isPress: Boolean)

class MainActivity : ComponentActivity() {
    private val port = getAvailablePort()
    private val nsd = NsdService(this, port)
    private val wss = WsServer(port, "my trollword", onOpen = { handshake ->
        nsd.unregister()
        val buttonsJson = handshake.getFieldValue("X-Trollbuttons")
        val buttons = Json.decodeFromString<List<ButtonInfo>>(buttonsJson)
        this.buttons.clear()
        this.buttons.addAll(buttons)
    }, onClose = {
        nsd.register()
        this.buttons.clear()
    })

    val buttons = mutableStateListOf<ButtonInfo>()

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        enableEdgeToEdge()

        nsd.register()
        wss.start()

        setContent {
            TrollboardTheme {
                Surface(
                    color = MaterialTheme.colorScheme.background,
                    contentColor = MaterialTheme.colorScheme.onBackground
                ) {
                    Screen()
                }
            }
        }
    }

    override fun onDestroy() {
        super.onDestroy()
        try {
            nsd.unregister()
            wss.stop()
        } catch (_: Exception) {
        }
    }

    @Composable
    fun Screen() {
        Column(
            Modifier
                .padding(WindowInsets.safeDrawing.asPaddingValues())
                .padding(top = 20.dp)
        ) {
            Text(
                "trollboard",
                fontFamily = ComicSans,
                style = MaterialTheme.typography.headlineLarge,
                modifier = Modifier.fillMaxWidth(),
                textAlign = TextAlign.Center
            )
            LazyVerticalGrid(
                columns = GridCells.Fixed(3),
                modifier = Modifier.fillMaxSize(),
                contentPadding = PaddingValues(20.dp),
            ) {
                items(buttons) { btn ->
                    Column {
                        Button(
                            modifier = Modifier
                                .aspectRatio(1f)
                                .padding(10.dp)
                                .pointerInteropFilter {
                                    fun send(isPress: Boolean) {
                                        val action = ButtonAction(btn.name, isPress)
                                        val actionJson = Json.encodeToString(action)
                                        wss.sendMessage(actionJson)
                                    }

                                    if (it.actionMasked == MotionEvent.ACTION_DOWN) send(true)
                                    else if (it.actionMasked == MotionEvent.ACTION_UP) send(false)
                                    true
                                },
                            onClick = {}
                        ) {
                            SvgImage(
                                data = btn.icon,
                                modifier = Modifier.size(120.dp)
                            )
                        }
                        Text(
                            btn.desc,
                            style = MaterialTheme.typography.bodyLarge,
                            textAlign = TextAlign.Center,
                            modifier = Modifier.fillMaxWidth()
                        )
                    }
                }
            }
        }
    }

    fun getAvailablePort(): Int {
        ServerSocket(0).use { socket ->
            return socket.localPort
        }
    }
}


