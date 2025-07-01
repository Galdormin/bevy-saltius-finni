#import
buttons as _

#defs
+header = \
    FlexNode{
        margin:{top:10px bottom:5px left:18px right:18px}
    }
    TextLine{}
    TextLineSize(12)
    TextLineFont({family:"m6x11"})
\

+base_text = \
    FlexNode{}

    "text"
        TextLine{}
        TextLineSize(8)
        TextLineFont({family:"m6x11"})
        TextLineColor(#EEFFAA)
\

+left = \
    +base_text{
        FlexNode{
            justify_main:FlexEnd
        }
    }
\

+right = \
    +base_text{
        FlexNode{
            justify_main:FlexStart
        }
    }
\

+grid = \
    GridNode{
        column_gap:12px
        grid_auto_flow:Row
        grid_template_columns: [(Count(2), 300px)]

    }
\

#scenes
"scene"
    // Root node covers the window.
    FlexNode{
        width:100vw height:100vh
        justify_main:Center
        justify_cross:Center
        flex_direction:Column
    }

    "created_by"
        +header{
            TextLineText("Created by")
        }

    ""
        +grid{}

        ""
            +left{
                "text"
                    TextLineText("Galdormin")
            }

        ""
            +right{
                "text"
                    TextLineText("Game Designer & Programmer")
            }


        ""
            +left{
                "text"
                    TextLineText("Nexia")
            }

        ""
            +right{
                "text"
                    TextLineText("Artist")
                }

    "assets"
        +header{
            TextLineText("Assets")
        }

    ""
        +grid{}

        ""
            +left{
                "text"
                    TextLineText("Button SFX")
            }

        ""
            +right{
                "text"
                    TextLineText("Jaszunio15")
            }

        ""
            +left{
                "text"
                    TextLineText("Music")
            }

        ""
            +right{
                "text"
                    TextLineText("Going Up by Ansimuz")
            }

    ""
        FlexNode{
            margin:{top:20px}
        }


    ""
        +button{
            ChangeMenuButton(Main)
            "text"
                TextLineText("Back")
        }
