"use client";

import Editor, { OnMount, loader } from "@monaco-editor/react";
import { useRef, useEffect, useState } from "react";

interface CodeEditorProps {
  value: string;
  onChange: (value: string) => void;
  language?: string;
  readOnly?: boolean;
}

export function CodeEditor({
  value,
  onChange,
  language = "go",
  readOnly = false,
}: CodeEditorProps) {
  const editorRef = useRef<Parameters<OnMount>[0] | null>(null);
  const vimModeRef = useRef<{ dispose: () => void } | null>(null);
  const statusBarRef = useRef<HTMLDivElement>(null);
  const [vimStatus, setVimStatus] = useState("-- NORMAL --");

  const handleEditorMount: OnMount = async (editor, monaco) => {
    editorRef.current = editor;

    // Configure Go language
    monaco.languages.register({ id: "go" });

    // Initialize Vim mode
    try {
      const { initVimMode } = await import("monaco-vim");
      
      if (statusBarRef.current) {
        vimModeRef.current = initVimMode(editor, statusBarRef.current);
      }
    } catch (error) {
      console.error("Failed to initialize Vim mode:", error);
    }

    // Focus editor
    editor.focus();
  };

  // Cleanup vim mode on unmount
  useEffect(() => {
    return () => {
      if (vimModeRef.current) {
        vimModeRef.current.dispose();
      }
    };
  }, []);

  // Re-initialize vim mode when switching between code/solution tabs
  useEffect(() => {
    const initVim = async () => {
      if (editorRef.current && statusBarRef.current && !readOnly) {
        // Dispose existing vim mode
        if (vimModeRef.current) {
          vimModeRef.current.dispose();
        }
        
        try {
          const { initVimMode } = await import("monaco-vim");
          vimModeRef.current = initVimMode(editorRef.current, statusBarRef.current);
        } catch (error) {
          console.error("Failed to re-initialize Vim mode:", error);
        }
      }
    };
    
    // Small delay to ensure editor is ready
    const timer = setTimeout(initVim, 100);
    return () => clearTimeout(timer);
  }, [readOnly]);

  return (
    <div className="h-full flex flex-col">
      <Editor
        height="calc(100% - 28px)"
        language={language}
        value={value}
        onChange={(val) => onChange(val || "")}
        onMount={handleEditorMount}
        theme="vs-dark"
        options={{
          fontSize: 14,
          fontFamily: "'JetBrains Mono', 'Fira Code', monospace",
          minimap: { enabled: false },
          lineNumbers: "on",
          scrollBeyondLastLine: false,
          automaticLayout: true,
          tabSize: 4,
          insertSpaces: false,
          readOnly,
          cursorStyle: "block",
          cursorBlinking: "solid",
        }}
      />
      {/* Vim Status Bar */}
      <div
        ref={statusBarRef}
        className="h-7 bg-zinc-900 text-emerald-400 text-xs px-3 flex items-center justify-center font-mono border-t border-zinc-700 font-semibold"
      />
    </div>
  );
}
