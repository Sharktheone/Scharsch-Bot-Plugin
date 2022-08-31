package de.scharschbot.plugin

import org.apache.http.auth.UsernamePasswordCredentials
import org.apache.http.client.methods.CloseableHttpResponse
import org.apache.http.client.methods.HttpPost
import org.apache.http.entity.StringEntity
import org.apache.http.impl.auth.BasicScheme
import org.apache.http.impl.client.HttpClients
import org.bukkit.Bukkit
import org.bukkit.command.CommandExecutor
import org.bukkit.event.EventHandler
import org.bukkit.event.Listener
import org.bukkit.event.entity.PlayerDeathEvent
import org.bukkit.event.player.AsyncPlayerChatEvent
import org.bukkit.event.player.PlayerAdvancementDoneEvent
import org.bukkit.event.player.PlayerJoinEvent
import org.bukkit.event.player.PlayerQuitEvent
import org.bukkit.plugin.java.JavaPlugin


class Plugin : JavaPlugin(), Listener, CommandExecutor {
    override fun onEnable() {
        super.onEnable()
        logger.info("ScharschBot Plugin Loaded!")
        Bukkit.getPluginManager().registerEvents(this,this)
        saveDefaultConfig()
    }
    override fun onDisable() {
        logger.info("ScharschBot Plugin Disabled - Bye see you next time")
    }
    private fun sendValues(Data: String){
        val httpClient = HttpClients.createDefault()
        logger.info(Data)

        try {
            val request = HttpPost(config.getString("URL"))


            val creds = UsernamePasswordCredentials(config.getString("User"),config.getString("Pass"))
            request.entity = StringEntity(Data)
            request.setHeader("Content-type", "application/json")
            request.addHeader(BasicScheme().authenticate(creds, request, null))

            val response: CloseableHttpResponse = httpClient.execute(request)
            logger.info(response.statusLine.statusCode.toString())
            if (response.statusLine.statusCode != 200) {
                logger.warning("Failure sending data to discord bot: " + response.statusLine.reasonPhrase)
            }
            response.close()
            httpClient.close()
        } catch (e: Exception) {
            logger.warning("Failed to send HTTP Request")
        }



    }
    @EventHandler
    fun chatMessage(event: AsyncPlayerChatEvent) {
        val chatJson = "{\"name\":\"" + event.player.name + "\", \"value\":\"" + event.message + "\", \"type\":\"chat\"}"
        sendValues(chatJson)
    }
    @EventHandler
    fun playerDeath(event: PlayerDeathEvent){
        val deathJson = "{\"name\":\"" + event.player.name + "\", \"value\":\"" + event.deathMessage + "\", \"type\":\"death\"}"
        sendValues(deathJson)
    }
    @EventHandler
    fun playerAdvancement(event: PlayerAdvancementDoneEvent){
        if(!event.advancement.key.key.contains("recipes/")) {
            val advancementJson = "{\"name\":\"" + event.player.name + "\", \"value\":\"" + event.advancement.key.key + "\", \"type\":\"advancement\"}"
            sendValues(advancementJson)
        }
    }
    @EventHandler
    fun playerJoin(event: PlayerJoinEvent){
        val joinJson = "{\"name\":\"" + event.player.name + "\", \"type\":\"join\"}"
        sendValues(joinJson)
    }
    @EventHandler
    fun playerQuit(event: PlayerQuitEvent){
        val quitJson = "{\"name\":\"" + event.player.name + "\", \"type\":\"quit\"}"
        sendValues(quitJson)
    }

}