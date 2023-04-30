package de.scharschbot.plugin

import io.papermc.paper.event.player.AsyncChatEvent
import org.bukkit.event.EventHandler
import org.bukkit.event.Listener
import org.bukkit.event.entity.PlayerDeathEvent
import org.bukkit.event.player.PlayerAdvancementDoneEvent
import org.bukkit.event.player.PlayerJoinEvent
import org.bukkit.event.player.PlayerQuitEvent

import org.bukkit.event.server.PluginDisableEvent
import org.bukkit.plugin.java.JavaPlugin
import java.util.logging.Logger
import java.io.File
import java.nio.file.Files

class Events(private val logger: Logger): Listener {
    init {
        val libName = "libscharsch_bot_plugin"
        var libExtension = ".so"

        val osName = System.getProperty("os.name")

        if (osName.contains("Windows")) {
            libExtension = ".dll"
        } else if (osName.contains("Mac")) {
            libExtension = ".dylib"
        }
        val libDir = Files.createTempDirectory("ScharschBot")
        libDir.toFile().deleteOnExit()
        val libFile = File(libDir.toFile(), libName)

        javaClass.classLoader.getResourceAsStream(libName + libExtension).use { input ->
            if (input == null) {
                throw RuntimeException("Could not find ScharschBot library $libName")
            }
            Files.copy(input, libFile.toPath())
        }
        System.load(libFile.absolutePath)
        logger.info("Loaded ScharschBot library $libName")

        logger.info("Initializing ScharschBot core")

        Thread {
            onInitialize()
        }.start() // TODO: Do threading in Rust
    }

    private external fun onInitialize()

    @EventHandler
    external fun onPlayerJoin(event: PlayerJoinEvent)

    @EventHandler
    external fun onPlayerLeave(event: PlayerQuitEvent)

    @EventHandler
    external fun onPlayerChat(event: AsyncChatEvent)

    @EventHandler
    external fun onPlayerDeath(event: PlayerDeathEvent)

    @EventHandler
    external fun onPlayerAdvancement(event: PlayerAdvancementDoneEvent)

    @EventHandler
    external fun onShutdown(event: PluginDisableEvent)
}