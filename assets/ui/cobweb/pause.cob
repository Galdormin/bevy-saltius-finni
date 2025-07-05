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
            StateButton<Menu>(None)
            "text"
                TextLineText("Continue")
        }

    ""
        +button{
            StateButton<Menu>(Settings)
            "text"
                TextLineText("Settings")
        }

    ""
        +button{
            StateButton<Screen>(Title)
            "text"
                TextLineText("Quit to title")
        }
