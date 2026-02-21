"use client";

import { useEffect, useState } from "react";
import Link from "next/link";
import { useParams } from "next/navigation";
import { Card, CardContent } from "@/components/ui/card";
import { Badge } from "@/components/ui/badge";
import { Button } from "@/components/ui/button";
import { ScrollArea } from "@/components/ui/scroll-area";
import { Separator } from "@/components/ui/separator";
import { fetchChapters, fetchExercises, Chapter, Exercise } from "@/lib/api";
import { useProgressStore } from "@/stores/progress";
import { cn } from "@/lib/utils";

export default function ChapterPage() {
  const params = useParams();
  const courseId = params.courseId as string;
  const chapterId = params.chapterId as string;

  const [chapter, setChapter] = useState<Chapter | null>(null);
  const [exercises, setExercises] = useState<Exercise[]>([]);
  const [chapters, setChapters] = useState<Chapter[]>([]);
  const [loading, setLoading] = useState(true);
  const { isComplete } = useProgressStore();

  useEffect(() => {
    async function loadData() {
      if (!courseId) return;
      try {
        const [chaptersData, exercisesData] = await Promise.all([
          fetchChapters(courseId),
          fetchExercises(courseId, chapterId),
        ]);
        setChapters(chaptersData);
        setChapter(chaptersData.find((c) => c.id === chapterId) || null);
        setExercises(exercisesData);
      } catch (error) {
        console.error("Failed to load chapter:", error);
      } finally {
        setLoading(false);
      }
    }
    loadData();
  }, [courseId, chapterId]);

  if (loading) {
    return (
      <div className="min-h-screen bg-background flex items-center justify-center">
        <div className="text-zinc-500 animate-pulse text-sm font-medium tracking-tight">Loading chapter...</div>
      </div>
    );
  }

  if (!chapter) {
    return (
      <div className="min-h-screen bg-background flex items-center justify-center">
        <div className="text-zinc-500 text-sm font-medium tracking-tight">Chapter not found</div>
      </div>
    );
  }

  const currentIndex = chapters.findIndex((c) => c.id === chapterId);
  const prevChapter = currentIndex > 0 ? chapters[currentIndex - 1] : null;
  const nextChapter = currentIndex < chapters.length - 1 ? chapters[currentIndex + 1] : null;

  const completedCount = exercises.filter((e) => isComplete(courseId, chapterId, e.id)).length;

  return (
    <div className="min-h-screen bg-background text-foreground selection:bg-primary selection:text-primary-foreground">
      {/* Header */}
      <header className="border-b border-border bg-background/80 backdrop-blur-md sticky top-0 z-50">
        <div className="container mx-auto px-6 h-14 flex items-center justify-between">
          <div className="flex items-center gap-4">
            <Link href={`/course/${courseId}`}>
              <Button variant="ghost" size="sm" className="text-zinc-500 hover:text-foreground -ml-3">
                ← Back
              </Button>
            </Link>
            <Separator orientation="vertical" className="h-4 bg-border" />
            <div className="flex items-baseline gap-3">
              <h1 className="text-sm font-semibold tracking-tight capitalize">
                {chapter.order}. {chapter.name.replace(/_/g, " ")}
              </h1>
              <span className="text-xs text-zinc-500 font-medium tracking-tight">
                {completedCount} / {exercises.length}
              </span>
            </div>
          </div>
          <div className="flex items-center gap-2">
            {prevChapter && (
              <Link href={`/course/${courseId}/chapter/${prevChapter.id}`}>
                <Button variant="outline" size="sm" className="text-zinc-500 hover:text-foreground">
                  ← Prev
                </Button>
              </Link>
            )}
            {nextChapter && (
              <Link href={`/course/${courseId}/chapter/${nextChapter.id}`}>
                <Button variant="outline" size="sm" className="text-zinc-500 hover:text-foreground">
                  Next →
                </Button>
              </Link>
            )}
          </div>
        </div>
      </header>

      {/* Exercises List */}
      <section className="container mx-auto px-6 py-12">
        <div className="max-w-3xl mx-auto">
          <h2 className="text-sm font-semibold tracking-tight text-zinc-500 uppercase mb-8">Exercises</h2>
          <div className="flex flex-col border-t border-border">
            {exercises.map((exercise, index) => {
              const completed = isComplete(courseId, chapterId, exercise.id);

              return (
                <Link
                  key={exercise.id}
                  href={`/course/${courseId}/chapter/${chapterId}/exercise/${exercise.id}`}
                  className={cn(
                    "flex items-center justify-between py-4 border-b border-border group hover:bg-zinc-900/30 transition-colors outline-none focus-visible:bg-zinc-900/50 px-2 -mx-2 rounded-sm",
                    completed && "opacity-60 hover:opacity-100"
                  )}
                >
                  <div className="flex items-center gap-4">
                    <div
                      className={cn(
                        "w-5 h-5 rounded flex items-center justify-center text-[10px] font-bold tracking-tighter transition-colors",
                        completed
                          ? "bg-primary text-primary-foreground"
                          : "bg-transparent border border-zinc-600 text-zinc-500 group-hover:border-zinc-400 group-hover:text-zinc-300"
                      )}
                    >
                      {completed ? "✓" : index + 1}
                    </div>
                    <div className="flex items-baseline gap-3">
                      <h3 className="text-sm font-medium tracking-tight capitalize group-hover:text-primary transition-colors">
                        {exercise.name.replace(/_/g, " ")}
                      </h3>
                      <p className="text-xs text-zinc-500 tracking-tight">
                        {exercise.type === "code" ? "Code" : "Quiz"}
                      </p>
                    </div>
                  </div>
                  <div className="text-zinc-500 opacity-0 -translate-x-2 group-hover:opacity-100 group-hover:translate-x-0 transition-all duration-300">
                    →
                  </div>
                </Link>
              );
            })}
          </div>
        </div>
      </section>
    </div>
  );
}
