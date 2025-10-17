#!/usr/bin/env python3
from crewai import Task

from crew_agents import judge

judge_task = Task(
    description="""
请评估双方的论点，并给出有理有据的裁决: {motion}，说明哪一方的论述更具说服力以及原因。
请以结构化的 Markdown 格式作答，包含以下部分：
1. 引言（Introduction）：
2. 论点分析（Arguments）：
3. 结论（Conclusion）：
4. 裁决（Verdict）：
5. 理由（Why）：
""".strip(),
    expected_output="一个有理有据的裁决，说明哪一方的论述更具说服力，以及原因。",
    agent=judge,
)
