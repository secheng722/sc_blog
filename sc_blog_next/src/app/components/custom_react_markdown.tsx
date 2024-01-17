import Markdown from "react-markdown";
import remarkGfm from "remark-gfm";
import {Prism as SyntaxHighlighter} from 'react-syntax-highlighter';
import {vscDarkPlus} from 'react-syntax-highlighter/dist/cjs/styles/prism';

const CustomReactMarkdown = ({content}: { content: string }) => {
    return (
        <Markdown className="prose lg:prose-xl" remarkPlugins={[[remarkGfm, {singleTilde: false}]]}
                  components={{
                      code({node, inline, className, children, ...props}: any) {
                          const match = /language-(\w+)/.exec(className || '');

                          return !inline && match ? (
                              <SyntaxHighlighter style={vscDarkPlus} PreTag="div" language={match[1]} {...props}>
                                  {String(children).replace(/\n$/, '')}
                              </SyntaxHighlighter>
                          ) : (
                              <code className={className} {...props}>
                                  {children}
                              </code>
                          );
                      },
                  }}
        >
            {content}
        </Markdown>
    )
}
export default CustomReactMarkdown;
