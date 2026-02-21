"use client";

import { useEffect, useState, useCallback } from "react";
import { useParams, useRouter } from "next/navigation";
import Link from "next/link";
import { Button } from "@/components/ui/button";
import { Separator } from "@/components/ui/separator";
import { Badge } from "@/components/ui/badge";
import { Tabs, TabsContent, TabsList, TabsTrigger } from "@/components/ui/tabs";
import { CodeEditor } from "@/components/code-editor";
import { ExerciseReadme } from "@/components/exercise-readme";
import { MultipleChoice } from "@/components/multiple-choice";
import { OutputPanel } from "@/components/output-panel";
import {
  fetchExercise,
  fetchExercises,
  fetchSolution,
  executeCode,
  ExerciseDetail,
  Exercise,
  ExecuteResult,
} from "@/lib/api";
import { useProgressStore } from "@/stores/progress";

export default function ExercisePage() {
  const params = useParams();
  const router = useRouter();
  const courseId = params.courseId as string;
  const chapterId = params.chapterId as string;
  const exerciseId = params.exerciseId as string;

  const [exercise, setExercise] = useState<ExerciseDetail | null>(null);
  const [exercises, setExercises] = useState<Exercise[]>([]);
  const [code, setCode] = useState("");
  const [output, setOutput] = useState<ExecuteResult | null>(null);
  const [isRunning, setIsRunning] = useState(false);
  const [showSolution, setShowSolution] = useState(false);
  const [solution, setSolution] = useState<string | null>(null);
  const [loading, setLoading] = useState(true);

  const { markComplete, isComplete } = useProgressStore();

  const getLanguage = () => {
    return courseId.includes("rust") ? "rust" : "go";
  };

  useEffect(() => {
    async function loadData() {
      if (!courseId) return;
      try {
        const [exerciseData, exercisesData] = await Promise.all([
          fetchExercise(courseId, chapterId, exerciseId),
          fetchExercises(courseId, chapterId),
        ]);
        setExercise(exerciseData);
        setExercises(exercisesData);
        setCode(exerciseData.starterCode || "");
        setOutput(null);
        setShowSolution(false);
        setSolution(null);
      } catch (error) {
        console.error("Failed to load exercise:", error);
      } finally {
        setLoading(false);
      }
    }
    loadData();
  }, [courseId, chapterId, exerciseId]);

  const handleRun = useCallback(async () => {
    if (!exercise || isRunning) return;
    setIsRunning(true);
    try {
      const result = await executeCode(code, getLanguage(), exercise.expectedOutput);
      setOutput(result);
      if (result.passed) {
        markComplete(courseId, chapterId, exerciseId);
      }
    } catch (error) {
      console.error("Execution failed:", error);
      setOutput({
        stdout: "",
        stderr: "Failed to execute code. Is the server running?",
        exitCode: 1,
        success: false,
        passed: false,
      });
    } finally {
      setIsRunning(false);
    }
  }, [code, exercise, courseId, chapterId, exerciseId, isRunning, markComplete]);

  const handleShowSolution = async () => {
    if (solution) {
      setShowSolution(!showSolution);
      return;
    }
    try {
      const sol = await fetchSolution(courseId, chapterId, exerciseId);
      setSolution(sol);
      setShowSolution(true);
    } catch (error) {
      console.error("Failed to fetch solution:", error);
    }
  };

  const handleMultipleChoiceAnswer = (correct: boolean) => {
    if (correct) {
      markComplete(courseId, chapterId, exerciseId);
    }
  };

  const goToNextExercise = () => {
    const currentIndex = exercises.findIndex((e) => e.id === exerciseId);
    if (currentIndex < exercises.length - 1) {
      router.push(`/course/${courseId}/chapter/${chapterId}/exercise/${exercises[currentIndex + 1].id}`);
    } else {
      router.push(`/course/${courseId}/chapter/${chapterId}`);
    }
  };

  const goToPrevExercise = () => {
    const currentIndex = exercises.findIndex((e) => e.id === exerciseId);
    if (currentIndex > 0) {
      router.push(`/course/${courseId}/chapter/${chapterId}/exercise/${exercises[currentIndex - 1].id}`);
    }
  };

  if (loading) {
    return (
      <div className="h-screen bg-background flex items-center justify-center">
        <div className="text-zinc-500 animate-pulse text-sm font-medium tracking-tight">Loading exercise...</div>
      </div>
    );
  }

  if (!exercise) {
    return (
      <div className="h-screen bg-background flex items-center justify-center">
        <div className="text-zinc-500 text-sm font-medium tracking-tight">Exercise not found</div>
      </div>
    );
  }

  const currentIndex = exercises.findIndex((e) => e.id === exerciseId);
  const completed = isComplete(courseId, chapterId, exerciseId);
  const lang = getLanguage();
  const fileExtension = lang === "rust" ? "rs" : "go";

  return (
    <div className="h-screen flex flex-col bg-background text-foreground selection:bg-primary selection:text-primary-foreground">
      {/* Header */}
      <header className="flex-none border-b border-border bg-background">
        <div className="px-4 h-12 flex items-center justify-between">
          <div className="flex items-center gap-4">
            <Link href={`/course/${courseId}/chapter/${chapterId}`}>
              <Button variant="ghost" size="sm" className="text-zinc-500 hover:text-foreground -ml-2">
                ← Back
              </Button>
            </Link>
            <Separator orientation="vertical" className="h-4 bg-border" />
            <div className="flex items-center gap-3">
              <h1 className="text-sm font-semibold tracking-tight capitalize">
                {exercise.name.replace(/_/g, " ")}
              </h1>
              {completed && (
                <Badge variant="outline" className="text-[10px] text-zinc-500 border-border uppercase tracking-wider px-1.5 rounded-sm">
                  Completed
                </Badge>
              )}
            </div>
          </div>
          <div className="flex items-center gap-2">
            <span className="text-xs text-zinc-500 font-medium tracking-tight px-2">
              {currentIndex + 1} / {exercises.length}
            </span>
            <Button
              variant="outline"
              size="sm"
              onClick={goToPrevExercise}
              disabled={currentIndex === 0}
              className="text-zinc-500 hover:text-foreground"
            >
              ←
            </Button>
            <Button
              variant="outline"
              size="sm"
              onClick={goToNextExercise}
              className="text-zinc-500 hover:text-foreground"
            >
              →
            </Button>
          </div>
        </div>
      </header>

      {/* Main Content */}
      <div className="flex-1 flex overflow-hidden bg-[#0A0A0A]">
        {/* Left Panel - Instructions */}
        <div className="w-1/2 flex flex-col border-r border-white/[0.05] bg-background">
          <ExerciseReadme content={exercise.readme} />
        </div>

        {/* Right Panel - Editor or Multiple Choice */}
        <div className="w-1/2 flex flex-col bg-[#0A0A0A]">
          {exercise.type === "code" ? (
            <>
              {/* Editor Tabs */}
              <Tabs defaultValue="code" className="flex-1 flex flex-col">
                <div className="flex-none border-b border-white/[0.05] px-4 pt-2 bg-[#0C0C0C]">
                  <TabsList variant="line" className="h-10 justify-start w-full bg-transparent p-0">
                    <TabsTrigger value="code" className="font-mono text-[11px] tracking-tight py-2 px-4 h-full data-[state=active]:bg-white/[0.02]">
                      main.{fileExtension}
                    </TabsTrigger>
                    {showSolution && (
                      <TabsTrigger value="solution" className="font-mono text-[11px] tracking-tight py-2 px-4 h-full data-[state=active]:bg-white/[0.02]">
                        solution.{fileExtension}
                      </TabsTrigger>
                    )}
                  </TabsList>
                </div>
                <TabsContent value="code" className="flex-1 m-0 data-[state=active]:flex data-[state=active]:flex-col border-none">
                  <div className="flex-1 bg-[#0A0A0A]">
                    <CodeEditor value={code} onChange={setCode} />
                  </div>
                </TabsContent>
                {showSolution && solution && (
                  <TabsContent value="solution" className="flex-1 m-0 data-[state=active]:flex data-[state=active]:flex-col border-none">
                    <div className="flex-1 bg-[#0A0A0A]">
                      <CodeEditor value={solution} onChange={() => {}} />
                    </div>
                  </TabsContent>
                )}
              </Tabs>

              {/* Action Bar */}
              <div className="flex-none border-t border-white/[0.05] px-6 py-4 flex items-center justify-between bg-[#0C0C0C]">
                <div className="flex items-center gap-3">
                  <Button
                    onClick={handleRun}
                    disabled={isRunning}
                    variant="default"
                    className="h-9 px-5 bg-white text-black hover:bg-zinc-200 font-semibold tracking-tight transition-all active:scale-[0.98]"
                  >
                    {isRunning ? (
                      <div className="flex items-center gap-2">
                        <div className="w-3 h-3 border-2 border-black/30 border-t-black rounded-full animate-spin" />
                        Running...
                      </div>
                    ) : "▶ Run Code"}
                  </Button>
                  <Button
                    variant="ghost"
                    onClick={handleShowSolution}
                    className="text-zinc-500 hover:text-white hover:bg-white/[0.05] font-medium transition-colors"
                  >
                    {showSolution ? "Hide Solution" : "Show Solution"}
                  </Button>
                </div>
                {output?.passed && (
                  <Button 
                    onClick={goToNextExercise} 
                    variant="secondary"
                    className="h-9 px-5 bg-zinc-900 border border-white/[0.1] text-white hover:bg-zinc-800 transition-all font-medium"
                  >
                    Next Exercise →
                  </Button>
                )}
              </div>

              {/* Output Panel */}
              <div className="flex-none h-64 border-t border-white/[0.05] overflow-hidden">
                <OutputPanel
                  stdout={output?.stdout || ""}
                  stderr={output?.stderr || ""}
                  passed={output?.passed}
                  expectedOutput={exercise.expectedOutput}
                  isLoading={isRunning}
                />
              </div>
            </>
          ) : (
            /* Multiple Choice */
            <div className="flex-1 flex flex-col bg-background">
              <div className="flex-1 overflow-auto">
                {exercise.multipleChoice && (
                  <MultipleChoice
                    question={exercise.multipleChoice.question}
                    answers={exercise.multipleChoice.answers}
                    onAnswer={handleMultipleChoiceAnswer}
                  />
                )}
              </div>
              {completed && (
                <div className="flex-none border-t border-border px-6 py-4 bg-background flex justify-end">
                  <Button onClick={goToNextExercise} variant="default">
                    Next Exercise →
                  </Button>
                </div>
              )}
            </div>
          )}
        </div>
      </div>
    </div>
  );
}
