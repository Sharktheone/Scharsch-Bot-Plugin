package de.scharschbot.plugin

import org.bukkit.Bukkit
import org.bukkit.command.CommandExecutor
import org.bukkit.event.Listener
import org.bukkit.plugin.java.JavaPlugin


class Plugin : JavaPlugin(), Listener, CommandExecutor {
    override fun onEnable() {
        super.onEnable()
        logger.info("ScharschBot Plugin Loaded!")
        Bukkit.getPluginManager().registerEvents(Events(logger), this)
        saveDefaultConfig()
    }
    override fun onDisable() {
        logger.info("ScharschBot Plugin Disabled - Bye see you next time")
    }

}