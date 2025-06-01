package dev.catte.trollboard.ui.component

import androidx.compose.foundation.Image
import androidx.compose.runtime.Composable
import androidx.compose.runtime.remember
import androidx.compose.ui.Modifier
import androidx.compose.ui.platform.LocalContext
import coil3.ImageLoader
import coil3.compose.rememberAsyncImagePainter
import coil3.request.ImageRequest
import coil3.svg.SvgDecoder

@Composable
fun SvgImage(data: String, modifier: Modifier = Modifier) {
    val context = LocalContext.current

    val imageLoader = remember {
        ImageLoader.Builder(context)
            .components {
                add(SvgDecoder.Factory())
            }
            .build()
    }

    val painter = rememberAsyncImagePainter(
        model = ImageRequest.Builder(context)
            .data(data.toByteArray())
            .build(),
        imageLoader = imageLoader
    )

    Image(
        painter = painter,
        contentDescription = null,
        modifier = modifier
    )
}