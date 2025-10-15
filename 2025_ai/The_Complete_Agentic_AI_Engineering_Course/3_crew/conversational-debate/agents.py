from crewai import Agent

proposer = Agent(
    role="正方",
    goal="一个有说服力的辩手",
    backstory="请就以下辩题提出一条立场明确的论点，支持辩题: {motion}",
    verbose=True,
)

opposer = Agent(
    role="反方",
    goal="一个善于陈述论点的辩手",
    backstory="请就以下辩题提出一条立场明确的论点，反对辩题: {motion}",
    verbose=True,
)

judge = Agent(
    role="裁判员,",
    goal="根据以下辩题的正反双方论点: {motion}，请仅依据双方所提出的论据，判断哪一方的论述更具说服力。",
    backstory="你是一位公正的评委，以不夹杂个人观点、仅根据论点本身的优劣进行评判而闻名。辩题是：{motion}",
    verbose=True,
)
