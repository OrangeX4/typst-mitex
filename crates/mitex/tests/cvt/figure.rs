use super::prelude::*;

#[test]
fn figure() {
    assert_debug_snapshot!(convert_text(r###"
    \begin{figure}[ht]
        \centering
        \includegraphics[width=0.5\textwidth]{example-image}
        \caption{This is an example image.}
        \label{fig:example}
    \end{figure}
    "###), @r###"
    Ok(
        "\n#figure(caption: [This is an example image.],)[\n\n#miteximage[\\[width=0.5 \\]];[example-image];\n\n\n];<fig:example>\n",
    )
    "###);
}

#[test]
fn table() {
    assert_debug_snapshot!(convert_text(r###"
    \begin{table}[ht]
        \centering
        \begin{tabular}{|c|c|}
            \hline
            \textbf{Name} & \textbf{Age} \\
            \hline
            John & 25 \\
            Jane & 22 \\
            \hline
        \end{tabular}
        \caption{This is an example table.}
        \label{tab:example}
    \end{table}
    "###), @r###"
    Ok(
        "\n#figure(caption: [This is an example table.],)[\n\n\n\n\n];<tab:example>\n",
    )
    "###);
}
