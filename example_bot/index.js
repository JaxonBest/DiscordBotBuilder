const { Client, Intents } = require('discord.js');
const fs = require('fs');
const client = new Client({ intents: [Intents.FLAGS.GUILDS] });

client.on('ready', () => {
    console.log(`Logged in as ${client.user.tag}.`);
});

client.on('interactionCreate', async interaction => {
    if (!interaction.isCommand()) return;

    if (interaction.commandName === 'ping') {
        await interaction.reply('Pong!');
    }
});

// Change this to be your token.
client.login('OTQ3MzQ1NzQxODQ2ODE4ODc2.GzEY60.8y8VmH2Z2yT6L45uXbIWHOAjSbfECtO-oZyBA0');