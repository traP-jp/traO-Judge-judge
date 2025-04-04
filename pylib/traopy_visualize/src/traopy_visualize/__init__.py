from traopy_visualize import _visualize
from traopy_visualize import _parser
import math

def visualize(
    schema: str,
    resp: str | None = None,
    title: str = "traOJudge Problem Procedure",
    use_graphviz: bool = True,
    element_size_coefficient: float = 1.0,
    graph_width_inch: int = 12,
    graph_height_inch: int = 8,
) -> None:
    visualizable = _parser.parse(schema, resp)
    _visualize.visualize_dag(
        visualizable.graph,
        visualizable.edge_labels,
        visualizable.node_styles,
        size_coefficient=math.sqrt(len(visualizable.node_styles)) / 15.0 * element_size_coefficient,
        title=title,
        use_graphviz=use_graphviz,
        graph_width_inch=graph_width_inch,
        graph_height_inch=graph_height_inch,
    )
