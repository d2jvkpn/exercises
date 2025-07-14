#!/usr/bin/env python3

import gradio as gr


def greet(name):
    return f"Hello {name}!"

view = gr.Interface(fn=greet, inputs="text", outputs="text")

view.launch()
