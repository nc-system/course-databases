// Ver extensiones instaladas
// code --list-extensions

// Config only 1 Container - Extensions
{
    "name": "NC Gases",
    
    "build": {
        "dockerfile": "../Dockerfile",
    },
}




// Config to several images + Extensions

{
    "name": "NC Gases",
    //"dockerFile": "Dockerfile",
    "dockerComposeFile": "../docker-compose.yml",
    "service": "api",
    "workspaceFolder": "/api",
    "shutdownAction": "stopCompose",

    "settings": {
        //"terminal.integrated.shell.linux": "/bin/bash",
        "editor.fontSize": 14,
        "workbench.colorTheme": "Dobri Next -A05- Jaguar",
        "workbench.iconTheme": "material-icon-theme"
    },

    "extensions": [
        "christian-kohler.npm-intellisense",
        "christian-kohler.path-intellisense",
        "cirlorm.mobileview",
        "CoenraadS.bracket-pair-colorizer",
        "cweijan.vscode-database-client2",
        //"denoland.vscode-deno",
        "dsznajder.es7-react-js-snippets",
        "eg2.vscode-npm-script",
        "esbenp.prettier-vscode",
        "formulahendry.auto-close-tag",
        "formulahendry.auto-rename-tag",
        "Gruntfuggly.todo-tree",
        "jundat95.react-native-snippet",
        "ms-azuretools.vscode-docker",
        "ms-vscode-remote.remote-containers",
        "ms-vscode-remote.remote-ssh",
        "ms-vscode-remote.remote-ssh-edit",
        "ms-vscode.vscode-typescript-next",
        "msjsdiag.vscode-react-native",
        "naumovs.color-highlight",
        "Orta.vscode-jest",
        "PKief.material-icon-theme",
        "ritwickdey.LiveServer",
        "sldobri.bunker",
        "steoates.autoimport",
        "styled-components.vscode-styled-components",
        "syler.sass-indented",
        //"TabNine.tabnine-vscode",
        "tomoki1207.pdf",
        //"Wscats.eno"
    ]
}





// Config to 1 image + Extension

{
    "name": "NC Gases",
    //"dockerFile": "Dockerfile",
    //"dockerComposeFile": "../docker-compose.yml",
    //"service": "api",
    //"workspaceFolder": "/api",
    //"shutdownAction": "stopCompose",

    "build": {
        "dockerfile": "../Dockerfile",
    },

    "settings": {
        //"terminal.integrated.shell.linux": "/bin/bash",
        "editor.fontSize": 14,
        "workbench.colorTheme": "Dobri Next -A05- Jaguar",
        "workbench.iconTheme": "material-icon-theme"
    },

    "extensions": [
        "13xforever.language-x86-64-assembly",
        "2gua.rainbow-brackets",
        "AbhijoyBasak.nestjs-files",
        "ashinzekene.nestjs",
        "christian-kohler.npm-intellisense",
        "christian-kohler.path-intellisense",
        "cirlorm.mobileview",
        "cschlosser.doxdocgen",
        "cweijan.vscode-database-client2",
        "denoland.vscode-deno",
        "DigitalBrainstem.javascript-ejs-support",
        "donjayamanne.githistory",
        "dsznajder.es7-react-js-snippets",
        "eamodio.gitlens",
        "eg2.vscode-npm-script",
        "esbenp.prettier-vscode",
        "FallenMax.mithril-emmet",
        "formulahendry.auto-close-tag",
        "formulahendry.auto-rename-tag",
        "formulahendry.code-runner",
        "Gruntfuggly.todo-tree",
        "jeff-hykin.better-cpp-syntax",
        "jundat95.react-native-snippet",
        "lit.lit-snippets",
        "mathiasfrohlich.Kotlin",
        "mhutchie.git-graph",
        "ms-azuretools.vscode-docker",
        "ms-vscode-remote.remote-containers",
        "ms-vscode-remote.remote-ssh",
        "ms-vscode-remote.remote-ssh-edit",
        "ms-vscode.cmake-tools",
        "ms-vscode.cpptools",
        "ms-vscode.cpptools-extension-pack",
        "ms-vscode.cpptools-themes",
        "ms-vscode.vscode-typescript-next",
        "ms-vsliveshare.vsliveshare",
        "ms-vsliveshare.vsliveshare-audio",
        "ms-vsliveshare.vsliveshare-pack",
        "msjsdiag.vscode-react-native",
        "naumovs.color-highlight",
        "Orta.vscode-jest",
        "PKief.material-icon-theme",
        "rangav.vscode-thunder-client",
        "RapidAPI.vscode-rapidapi-client",
        "ritwickdey.LiveServer",
        "sldobri.bunker",
        "steoates.autoimport",
        "styled-components.vscode-styled-components",
        "svelte.svelte-vscode",
        "syler.sass-indented",
        "TabNine.tabnine-vscode",
        "tomoki1207.pdf",
        "twxs.cmake",
        "usernamehw.errorlens",
        "Wscats.eno"
    ]
}
