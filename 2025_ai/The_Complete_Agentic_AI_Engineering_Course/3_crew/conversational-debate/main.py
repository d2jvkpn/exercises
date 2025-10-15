import time

from agents import proposer, opposer, judge
from tasks import judge_task

from dotenv import load_dotenv
from crewai import Agent, Task, Crew
from crewai.memory import LongTermMemory
import gradio as gr
from gradio import themes

start_prompt = "开始辩论，先发表你的支持立场的第一条论点: '{motion}'."
debat_prompt = """对方刚刚说: '{last_debug}'.
请针对这段发言提出有力的反驳，继续支持本方立场.
以下是完整的辩论记录: {debate_log}"""

def format_as_chat(message, chat_history=[], role="user"):
    chat_history.append({"role": role, "content": message})
    return chat_history

def format_as_chat_history(messages):
    return [
        {"role": "user" if i % 2 == 0 else "assistant", "content": message}
        for i, message in enumerate(messages)
    ]

def debate(motion="Being vegan is better for the environment", MAX_ROUNDS=4):
    print("Hello from debate-prep-ai!")
    load_dotenv(override=True)

    debate_log = []
    yield format_as_chat("Starting debate...", [], "assistant")

    print("Iterating now...")
    turn = "proposer"
    for i in range(MAX_ROUNDS):
        agent = proposer if i % 2 == 0 else opposer
        turn = "proposer 正方" if i % 2 == 0 else "opposer 反方"

        task = Task(
            description=start_prompt if not debate_log else debat_prompt,
            expected_output="A single sentence argument in favor of the motion: {motion}",
            agent=agent,
        )

        result = Crew(agents=[agent], tasks=[task]).kickoff(inputs={
            'motion': motion,
            'last_debug': debate_log[-1]['content'] if len(debate_log) > 0 else "",
            'debate_log': "\n\n".join([m['content'] for m in debate_log]),
        })

        # print(result.raw)
        message = f"## {turn.capitalize()}: \n{result}"
        debate_log.append({"role": "user" if i % 2 == 0 else "assistant", "content": ""})

        for character in message:
            debate_log[-1]['content'] += character
            time.sleep(0.005)
            yield debate_log

    # Final Verdict by Judge
    debate_summary = "\n\n".join(
        [
            (f"proposer: {point}" if i % 2 == 0 else f"opposer: {point}")
            for i, point in enumerate(debate_log)
        ]
    )

    yield format_as_chat(
        f"********************************\n\n ### Arguments Completed. \n\n Judging now ...\n\n********************************",
        debate_log,
        "assistant",
    )

    final_crew = Crew(tasks=[judge_task], agents=[judge], verbose=True)
    verdict = final_crew.kickoff(inputs={"motion": motion, "debate_summary": debate_summary})

    debate_log.append({"role": "assistant", "content": ""})
    for character in f"👨‍⚖️ FINAL VERDICT ARRIVED\n\n\n\n {verdict}":
        debate_log[-1]['content'] += character
        time.sleep(0.005)
        yield debate_log

intro_markdown = """
# Debate Prep AI

This is a simple Agentic-AI developed using CrewAI that helps you prepare for a debate. It uses 3 
agents: the proposer, the opposer and the judge. Use this app to understand what arguments (and 
counter-arguments) you can place during the debate."

## How to use:
1. Enter the motion of the debate in the text box below.
2. Click the 'Debate' button to start the debate.
3. The AI will generate arguments for your side and evaluate the arguments of the other side.
4. The AI will give you a verdict on which side was more persuasive and why.
5. Confused where to begin? Try asking `AI is bad for humans` and click on *Debate* button.
""".strip()

motion_info = """
This is the motion of the debate. It is the statement that you want to debate about. For e.g. you 
could say Being vegan is better for the environment
""".strip()

slider_info = """
This is the number of rounds of the debate. The more rounds, the more detailed the debate will be.
""".strip()

css = """
#chatbot {
  height: 100%;
  min-height: 75vh;
  max-width: 70vw;
}
"""

def renderInterface():
    with gr.Blocks(
        theme=themes.Default(primary_hue="blue"),
        fill_height=True,
        css=css,
   ) as ui:
        with gr.Row():
            with gr.Column(scale=1):
                gr.Markdown(intro_markdown)

                max_rounds = gr.Slider(
                    label="Number of Rounds",
                    value=4,
                    minimum=1,
                    maximum=10,
                    step=1,
                    info=slider_info,
                    visible=True,
                )

                motion = gr.Textbox(
                    label="Motion",
                    placeholder="What do you want to debate about?",
                    info=motion_info,
                    submit_btn="Debate",
                )

            with gr.Column(scale=2):
                chat = gr.Chatbot(label="Debate Log", value=[], type="messages", elem_id="chatbot")

            motion.submit(debate, inputs=[motion, max_rounds], outputs=[chat])

    ui.launch()

if __name__ == "__main__":
    renderInterface()
