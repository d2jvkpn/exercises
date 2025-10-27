import os, json
from pathlib import Path
from datetime import datetime
from typing import List, Any

from .tools.push_tool import PushNotificationTool

from pydantic import BaseModel, Field
from crewai import Agent, Crew, Process, Task
from crewai.project import CrewBase, agent, crew, task
from crewai.agents.agent_builder.base_agent import BaseAgent
from crewai_tools import SerperDevTool
from crewai.memory import LongTermMemory, ShortTermMemory, EntityMemory
from crewai.memory.storage.rag_storage import RAGStorage
from crewai.memory.storage.ltm_sqlite_storage import LTMSQLiteStorage

def now():
    return datetime.now().astimezone().strftime("%FT%T%:z")

def serialize_output(output: Any) -> dict:
    # 1) 官方 TaskOutput
    if hasattr(output, "to_dict") and callable(output.to_dict):
        return output.to_dict()  # 包含 raw/json_dict/pydantic 等（若有）

    # 2) Pydantic 模型（有时你直接拿到的是 BaseModel）
    if hasattr(output, "model_dump") and callable(output.model_dump):
        return output.model_dump()

    # 3) 已是可 JSON 的原生类型
    if isinstance(output, (dict, list, int, float, bool)) or output is None:
        return {"raw": output}

    # 4) 兜底：转成字符串
    return { "raw": str(output) }

def step_logger(output, *args, **kwargs):
    record = {
        "timestamp": now(),
        "step_output": str(output).replace("\\'", "'").replace('\\"', '"').replace("\\\\n", '\\n'),
    }

    with open("logs/steps.jsonl", "a", encoding="utf-8") as f:
        f.write(json.dumps(record, ensure_ascii=False) + "\n")

def task_logger(output, *args, **kwargs):
    record = {
        "timestamp": now(),
        "task_output": serialize_output(output),
    }

    with open("logs/tasks.jsonl", "a", encoding="utf-8") as f:
        f.write(json.dumps(record, ensure_ascii=False) + "\n")

class TrendingCompany(BaseModel):
    """A company that is in the news and attracting attetnion"""
    name: str = Field(descriprion="Company name")
    ticker: str = Field(descriprion="Stock ticker symbol")
    reason: str = Field(descriprion="Reason this company is trending in the news")

class TrendingCompanyList(BaseModel):
    """List of multiple trending companies that are in the news"""
    companies: List[TrendingCompany] = Field(description="List of companies trending in the news")

class TrendingCompanyResearch(BaseModel):
    """Detailed research on a company"""
    name: str = Field(description="Company name")
    market_position: str = Field(description="Current market position and competitive analysis")
    future_outlook: str = Field(description="Future outlook and growth prospects")

    investment_potential: str = Field(
        description="Investment potential and suitability for investment",
    )

class TrendingCompanyResearchList(BaseModel):
    """A list of detailed research on all the companies"""
    companies: List[TrendingCompanyResearch] = Field(
        description="Comprehensive research on all trending companies",
    )

@CrewBase
class StockPicker():
    """StockPicker crew"""

    agents: List[BaseAgent]
    tasks: List[Task]

    @agent
    def trending_company_finder(self) -> Agent:
        return Agent(
            config=self.agents_config['trending_company_finder'],
            #verbose=True,
            tools=[SerperDevTool()],
            memory=True,
        )

    @agent
    def financial_researcher(self) -> Agent:
        return Agent(
            config=self.agents_config['financial_researcher'],
            #verbose=True,
            tools=[SerperDevTool()],
        )

    @agent
    def stock_picker(self) -> Agent:
        return Agent(
            config=self.agents_config['stock_picker'],
            #verbose=True,
            tools=[PushNotificationTool()],
            memory=True,
        )

    @task
    def find_trending_companies(self) -> Task:
        return Task(
            config=self.tasks_config['find_trending_companies'],
            output_pydantic=TrendingCompanyList,
        )

    @task
    def research_trending_companies(self) -> Task:
        return Task(
            config=self.tasks_config['research_trending_companies'],
            output_pydantic=TrendingCompanyResearchList,
        )

    @task
    def pick_best_company(self) -> Task:
        return Task(config=self.tasks_config['pick_best_company'])

    @crew
    def crew(self) -> Crew:
        """Creates the StockPicker crew"""

        memory_dir = Path("data") / "memory"

        manager = Agent(
            config=self.agents_config['manager'],
            allow_delegation=True,
        )

        embedder_config = {
            "provider": "openai",
            "config": {
                "model_name": "text-embedding-3-small",
                "api_key": os.environ["OPENAI_API_KEY"],
            },
        }

        short_term_memory = ShortTermMemory(
            storage=RAGStorage(
                embedder_config=embedder_config,
                type="short_term",
                path=memory_dir,
            ),
        )

        long_term_memory = LongTermMemory(
            storage=LTMSQLiteStorage(db_path=memory_dir / "long_term_memory.db"),
        )

        entity_memory = EntityMemory(
            storage=RAGStorage(
                embedder_config=embedder_config,
                type="short_term",
                path=memory_dir,
            ),
        )

        log_dir = Path("logs")
        log_dir.mkdir(parents=True, exist_ok=True)
        data_dir = Path("data")
        data_dir.mkdir(parents=True, exist_ok=True)

        return Crew(
            agents=self.agents,
            tasks=self.tasks,
            process=Process.hierarchical, # Process.sequential,
            verbose=True,
            manager_agent=manager,
            #manager_llm="xx",

            memory=True,
            short_term_memory=short_term_memory,
            long_term_memory=long_term_memory,
            entity_memory=entity_memory,

            output_log_file="logs/crew_log.json",
            step_callback=step_logger,
            task_callback=task_logger,
        )
