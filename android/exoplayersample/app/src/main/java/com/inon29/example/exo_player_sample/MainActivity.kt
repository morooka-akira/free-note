package com.inon29.example.exo_player_sample

import android.net.Uri
import android.os.Bundle
import android.util.Log
import androidx.appcompat.app.AppCompatActivity
import androidx.databinding.DataBindingUtil
import com.google.android.exoplayer2.ExoPlayer
import com.google.android.exoplayer2.MediaItem
import com.google.android.exoplayer2.Player
import com.google.android.exoplayer2.source.ConcatenatingMediaSource
import com.google.android.exoplayer2.source.ProgressiveMediaSource
import com.google.android.exoplayer2.upstream.DefaultHttpDataSource
import com.google.android.exoplayer2.util.Util
import com.inon29.example.exo_player_sample.databinding.ActivityMainBinding

class MainActivity : AppCompatActivity(), Player.Listener {

    private lateinit var player: ExoPlayer
    private val binding: ActivityMainBinding by lazy {
        DataBindingUtil.setContentView(this, R.layout.activity_main)
    }
    private var index = 0

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        player = ExoPlayer.Builder(this).build()
        binding.playerView.player = player
        // 再生時、停止、などのタイミングでにコントローラを表示するか？
        binding.playerView.useController = true
        binding.dummyButton.setOnClickListener {
            index = (index + 1) % 3
            player.seekTo(index, 0)
        }
        val uriVideo: Uri = Uri.parse("https://commondatastorage.googleapis.com/gtv-videos-bucket/sample/BigBuckBunny.mp4")
        val uri = Uri.parse(
            "android.resource://${packageName}/${R.raw.launch_movie}"
        )
        val baseDataSourceFactory = DefaultHttpDataSource.Factory().apply {
            setUserAgent(Util.getUserAgent(this@MainActivity, getString(R.string.app_name)))
        }
        val mediaSource = ProgressiveMediaSource.Factory(baseDataSourceFactory)
            .createMediaSource(MediaItem.fromUri(Uri.parse(
                "android.resource://${packageName}/${R.raw.launch_movie}"
            )))
        val mediaSource2 = ProgressiveMediaSource.Factory(baseDataSourceFactory)
            .createMediaSource(MediaItem.fromUri(Uri.parse(
                "android.resource://${packageName}/${R.raw.launch_movie2}"
            )))
        val mediaSource3 = ProgressiveMediaSource.Factory(baseDataSourceFactory)
            .createMediaSource(MediaItem.fromUri(Uri.parse(
                "android.resource://${packageName}/${R.raw.launch_movie3}"
            )))
        val concatenatingMediaSource = ConcatenatingMediaSource(mediaSource, mediaSource2, mediaSource3)
//        player.setMediaSource(concatenatingMediaSource)

        player.setMediaItems(listOf(
            MediaItem.fromUri(Uri.parse(
                "android.resource://${packageName}/${R.raw.launch_movie}"
            )),
            MediaItem.fromUri(Uri.parse(
                "android.resource://${packageName}/${R.raw.launch_movie2}"
            )),
            MediaItem.fromUri(Uri.parse(
                "android.resource://${packageName}/${R.raw.launch_movie3}"
            )),
        ))

        player.volume = 1f
        player.prepare()
        this.player.addListener(this)

        // 自動再生
        player.playWhenReady = true
        player.repeatMode = Player.REPEAT_MODE_ONE
    }

    override fun onPlaybackStateChanged(playbackState: Int) {
        super.onPlaybackStateChanged(playbackState)
        when (playbackState) {
            Player.STATE_BUFFERING -> {
                Log.d("onPlaybackStateChanged", "Player.STATE_BUFFERING")
            }
            Player.STATE_ENDED -> {
                Log.d("onPlaybackStateChanged", "Player.STATE_ENDED")
            }
            Player.STATE_IDLE -> {
                Log.d("onPlaybackStateChanged", "Player.STATE_IDLE")
            }
            Player.STATE_READY -> {
                Log.d("onPlaybackStateChanged", "Player.STATE_READY")
            }
        }
    }
}
