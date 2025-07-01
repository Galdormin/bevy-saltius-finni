#import
buttons as _

#scenes
"scene"
    // Root node covers the window.
    FlexNode{
        width:100vw height:100vh
        justify_main:Center
        justify_cross:Center
        flex_direction:Column
    }

    "header"
        FlexNode{
            margin:{top:8px bottom:8px left:8px right:8px}
        }
        TextLine{text:"Game Paused" font:{family:"Monogram"} size:23}

    ""
        +button{
            ChangeMenuButton(None)
            "text"
                TextLineText("Continue")
        }

    ""
        +button{
            ChangeMenuButton(Settings)
            "text"
                TextLineText("Settings")
        }

    ""
        +button{
            ChangeScreenButton(Title)
            "text"
                TextLineText("Quit to title")
        }
