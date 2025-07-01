#manifest
"ui/cobweb/fonts.cob" as fonts
"ui/cobweb/buttons.cob" as buttons

#import
buttons as _

#commands
LoadImages[
    "icons/banner.png"
]

#scenes
"scene"
    // Root node covers the window.
    FlexNode{
        width:100vw height:100vh
        justify_main:Center
        justify_cross:Center
        flex_direction:Column
    }

    // "header"
    //     FlexNode{
    //         margin:{top:8px bottom:8px left:8px right:8px}
    //     }
    //     TextLine{text:"Saltius Finni" font:{family:"Monogram"} size:23}

    "header"
        FlexNode{
            margin:{top:8px bottom:8px left:8px right:8px}
        }
        LoadedImageNode{
            image: "icons/banner.png"
        }

    ""
        +button{
            ChangeScreenButton(Gameplay)
            "text"
                TextLineText("Play")
        }

    ""
        +button{
            ChangeMenuButton(Settings)
            "text"
                TextLineText("Settings")
        }

    ""
        +button{
            ChangeMenuButton(Credits)
            "text"
                TextLineText("Credits")
        }

    ""
        +button{
            QuitButton
            "text"
                TextLineText("Quit")
        }
