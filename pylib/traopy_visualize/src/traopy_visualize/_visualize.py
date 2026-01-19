import networkx as nx
import matplotlib.pyplot as plt

def visualize_dag(
    graph: nx.DiGraph,
    edge_labels: dict[tuple[str, str], str],
    node_styles: dict[str, dict[str, str | int]],
    title: str,
    use_graphviz: bool,
    graph_width_inch: int = 12,
    graph_height_inch: int = 8,
    size_coefficient: float = 1.0,
) -> None:
    if use_graphviz:
        pos = nx.nx_agraph.graphviz_layout(
            graph,
            prog="dot",
            args="-Grankdir=LR")
    else:
        pos = nx.spring_layout(graph)
    shapes = set(style["shape"] for style in node_styles.values())
    for shape in shapes:
        nodelist = [node for node, style in node_styles.items() if style["shape"] == shape]
        node_color = [node_styles[node]["color"] for node in nodelist]
        node_size = [node_styles[node]["size"] for node in nodelist]
        nx.draw_networkx_nodes(graph, pos,
                            nodelist=nodelist,
                            node_color=node_color,
                            node_size=[size_coefficient * size for size in node_size],
                            node_shape=shape)
    nx.draw_networkx_labels(
        graph,
        pos,
        font_size=10.0 * size_coefficient,
        font_color="black",
        font_weight="bold",
    )
    nx.draw_networkx_edges(
        graph,
        pos,
        edge_color="black",
        width=1.0 * size_coefficient,
        alpha=0.8 * size_coefficient,
        arrowsize=50.0 * size_coefficient,
    )
    nx.draw_networkx_edge_labels(
        graph,
        pos,
        edge_labels=edge_labels,
        rotate=False,
        font_size=6.0 * size_coefficient,
    )
    plt.gcf().set_size_inches(graph_width_inch, graph_height_inch)
    plt.title(title)
    plt.axis("off")
    plt.show()
