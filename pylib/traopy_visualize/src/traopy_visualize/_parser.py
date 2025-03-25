import enum
import json
import networkx as nx

class _JudgeStatus(enum.Enum):
    AC = enum.auto()
    WA = enum.auto()
    TLE = enum.auto()
    MLE = enum.auto()
    OLE = enum.auto()
    RE = enum.auto()
    CE = enum.auto()
    WE = enum.auto()
    Hidden = enum.auto()
    EarlyExit = enum.auto()

class _Node(enum.Enum):
    STATIC_TEXT = enum.auto()
    RUNTIME_TEXT = enum.auto()
    EMPTY_DIRECTORY = enum.auto()
    SCRIPT = enum.auto()
    EXECUTION = enum.auto()

class _Edge:
    src: str
    dst: str
    label: str
    def __init__(self, src: str, dst: str, label: str) -> None:
        self.src = src
        self.dst = dst
        self.label = label

class _Visualizable:
    graph: nx.DiGraph
    edge_labels: dict[tuple[str, str], str]
    node_styles: dict[str, dict[str, str | int]]
    
    def __init__(self, graph: nx.DiGraph, edge_labels: dict[tuple[str, str], str], node_styles: dict[str, dict[str, str | int]]) -> None:
        self.graph = graph
        self.edge_labels = edge_labels
        self.node_styles = node_styles

def parse(schema: str, resp: str | None) -> _Visualizable:
    schema = json.loads(schema)
    nodes: dict[str, _Node] = {}
    edges: list[_Edge] = []
    judge_status: dict[str, _JudgeStatus | None] = {}

    for resource in schema['resources']:
        if 'TextFile' in resource:
            nodes[resource['TextFile']['name']] = _Node.STATIC_TEXT
        elif 'RuntimeTextFile' in resource:
            nodes[resource['RuntimeTextFile']['name']] = _Node.RUNTIME_TEXT
        elif 'EmptyDirectory' in resource:
            nodes[resource['EmptyDirectory']['name']] = _Node.EMPTY_DIRECTORY
    for execution in schema['executions']:
        nodes[execution['name']] = _Node.EXECUTION
        judge_status[execution['name']] = None
        for dep in execution['dependencies']:
            edges.append(_Edge(src=dep['ref_to'], dst=execution['name'], label=dep['envvar_name']))
        edges.append(_Edge(src=execution['script_name'], dst=execution['name'], label='script'))
    for script in schema['scripts']:
        nodes[script['name']] = _Node.SCRIPT
    
    if resp is not None:
        resp = json.loads(resp)
        for (name, result) in resp.items():
            if result == "EarlyExit":
                    status = _JudgeStatus.EarlyExit
            elif 'Displayable' in result['ExecutionResult']:
                if result['ExecutionResult']['Displayable']['status'] == 'AC':
                    status = _JudgeStatus.AC
                elif result['ExecutionResult']['Displayable']['status'] == 'WA':
                    status = _JudgeStatus.WA
                elif result['ExecutionResult']['Displayable']['status'] == 'TLE':
                    status = _JudgeStatus.TLE
                elif result['ExecutionResult']['Displayable']['status'] == 'MLE':
                    status = _JudgeStatus.MLE
                elif result['ExecutionResult']['Displayable']['status'] == 'OLE':
                    status = _JudgeStatus.OLE
                elif result['ExecutionResult']['Displayable']['status'] == 'RE':
                    status = _JudgeStatus.RE
                elif result['ExecutionResult']['Displayable']['status'] == 'CE':
                    status = _JudgeStatus.CE
                elif result['ExecutionResult']['Displayable']['status'] == 'WE':
                    status = _JudgeStatus.WE
            else:
                status = _JudgeStatus.Hidden
            judge_status[name] = status
    
    graph = nx.DiGraph()
    for node in nodes:
        graph.add_node(node)
    for edge in edges:
        graph.add_edge(edge.src, edge.dst)
    edge_labels = {}
    for edge in edges:
        edge_labels[(edge.src, edge.dst)] = edge.label
    node_styles = {}
    for node in nodes:
        if nodes[node] == _Node.STATIC_TEXT:
            node_styles[node] = {'shape': 'h', 'color': 'blue', 'size': 300.0}
        elif nodes[node] == _Node.RUNTIME_TEXT:
            node_styles[node] = {'shape': 'h', 'color': 'blue', 'size': 300.0}
        elif nodes[node] == _Node.EMPTY_DIRECTORY:
            node_styles[node] = {'shape': 'h', 'color': 'blue', 'size': 300.0}
        elif nodes[node] == _Node.SCRIPT:
            node_styles[node] = {'shape': 'h', 'color': 'purple', 'size': 300.0}
        elif nodes[node] == _Node.EXECUTION:
            if judge_status[node] == _JudgeStatus.AC:
                node_styles[node] = {'shape': 'o', 'color': 'green', 'size': 1000.0}
            elif judge_status[node] == _JudgeStatus.WA:
                node_styles[node] = {'shape': 'o', 'color': 'orange', 'size': 1000.0}
            elif judge_status[node] == _JudgeStatus.TLE:
                node_styles[node] = {'shape': 'o', 'color': 'orange', 'size': 1000.0}
            elif judge_status[node] == _JudgeStatus.MLE:
                node_styles[node] = {'shape': 'o', 'color': 'orange', 'size': 1000.0}
            elif judge_status[node] == _JudgeStatus.OLE:
                node_styles[node] = {'shape': 'o', 'color': 'orange', 'size': 1000.0}
            elif judge_status[node] == _JudgeStatus.RE:
                node_styles[node] = {'shape': 'o', 'color': 'orange', 'size': 1000.0}
            elif judge_status[node] == _JudgeStatus.CE:
                node_styles[node] = {'shape': 'o', 'color': 'orange', 'size': 1000.0}
            elif judge_status[node] == _JudgeStatus.WE:
                node_styles[node] = {'shape': 'o', 'color': 'red', 'size': 1000.0}
            elif judge_status[node] == _JudgeStatus.Hidden:
                node_styles[node] = {'shape': 'o', 'color': 'gray', 'size': 1000.0}
            elif judge_status[node] == _JudgeStatus.EarlyExit:
                node_styles[node] = {'shape': 'o', 'color': 'lightseagreen', 'size': 1000.0}
            else:
                node_styles[node] = {'shape': 'o', 'color': 'olivedrab', 'size': 1000.0}
    return _Visualizable(graph=graph, edge_labels=edge_labels, node_styles=node_styles)