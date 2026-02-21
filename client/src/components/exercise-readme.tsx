"use client";

import ReactMarkdown from "react-markdown";
import remarkGfm from "remark-gfm";
import rehypeHighlight from "rehype-highlight";
import "highlight.js/styles/github-dark.css";

interface ExerciseReadmeProps {
  content: string;
}

export function ExerciseReadme({ content }: ExerciseReadmeProps) {
  return (
    <div className="prose prose-invert prose-lg max-w-none p-8 overflow-auto h-full bg-background scrollbar-thin scrollbar-track-zinc-950 scrollbar-thumb-zinc-800 selection:bg-primary/30">
      <ReactMarkdown
        remarkPlugins={[remarkGfm]}
        rehypePlugins={[rehypeHighlight]}
        components={{
          // Custom image rendering with max width
          img: ({ src, alt }) => (
            <img
              src={src}
              alt={alt || ""}
              className="max-w-full rounded-xl border border-border"
            />
          ),
          // Style code blocks
          pre: ({ children }) => (
            <pre className="bg-black rounded-lg p-4 overflow-x-auto border border-border text-[13px] leading-relaxed">
              {children}
            </pre>
          ),
          // Style inline code
          code: ({ className, children, ...props }) => {
            const isInline = !className;
            if (isInline) {
              return (
                <code className="bg-white/10 text-foreground px-1 py-0.5 rounded-sm font-medium tracking-tight before:content-[''] after:content-['']" {...props}>
                  {children}
                </code>
              );
            }
            return <code className={className} {...props}>{children}</code>;
          },
        }}
      >
        {content}
      </ReactMarkdown>
    </div>
  );
}
