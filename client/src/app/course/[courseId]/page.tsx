"use client";

import { useEffect, useState } from "react";
import Link from "next/link";
import { useParams } from "next/navigation";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { Progress } from "@/components/ui/progress";
import { Badge } from "@/components/ui/badge";
import { ScrollArea } from "@/components/ui/scroll-area";
import { fetchChapters, fetchExercises, Chapter, Exercise } from "@/lib/api";
import { useProgressStore } from "@/stores/progress";

export default function CoursePage() {
  const params = useParams();
  const courseId = params.courseId as string;

  const [chapters, setChapters] = useState<Chapter[]>([]);
  const [courseName, setCourseName] = useState("");
  const [chapterExercises, setChapterExercises] = useState<Record<string, Exercise[]>>({});
  const [loading, setLoading] = useState(true);
  const { getChapterProgress } = useProgressStore();

  useEffect(() => {
    async function loadData() {
      if (!courseId) return;
      try {
        const chaptersData = await fetchChapters(courseId);
        setChapters(chaptersData);
        setCourseName(courseId.replace(/-/g, " ")); // rudimentary parsing

        // Load exercises for each chapter to calculate progress
        const exercisesMap: Record<string, Exercise[]> = {};
        await Promise.all(
          chaptersData.map(async (chapter) => {
            const exercises = await fetchExercises(courseId, chapter.id);
            exercisesMap[chapter.id] = exercises;
          })
        );
        setChapterExercises(exercisesMap);
      } catch (error) {
        console.error("Failed to load chapters:", error);
      } finally {
        setLoading(false);
      }
    }
    loadData();
  }, [courseId]);

  const getChapterIcon = (chapterName: string): string => {
    const icons: Record<string, string> = {
      intro: "🚀",
      variables: "📦",
      functions: "⚡",
      ownership: "🛡️",
      structs: "🏗️",
      enums: "🔀",
      interfaces: "🔌",
      errors: "⚠️",
      loops: "🔄",
      collections: "📚",
      slices: "📚",
      maps: "🗺️",
      traits: "🧬",
      advanced_functions: "🎯",
      iterators_closures: "🔄",
      concurrency: "📡",
      smart_pointers: "🧠",
      pointers: "👆",
      local_development: "💻",
      cargo: "💻",
      channels: "📡",
      mutexes: "🔒",
      generics: "🧬",
      go_facts: "📖",
    };
    const key = chapterName.toLowerCase().replace(/ /g, "_").replace(/&/g, "").replace(/\s+/g, "_");
    // additional replacement for "ownership_borrowing" to match just "ownership"
    if (key.includes("ownership")) return icons.ownership;
    if (key.includes("enum")) return icons.enums;
    if (key.includes("error")) return icons.errors;
    return icons[key] || "📘";
  };

  const getCourseIcon = (id: string): React.ReactNode => {
      if (id.includes("go")) return (
        <img src="https://go.dev/blog/go-brand/Go-Logo/PNG/Go-Logo_Blue.png" alt="Go Logo" className="w-5 h-5 object-contain" />
      );
      if (id.includes("rust")) return (
        <img src="https://rust-lang.org/logos/rust-logo-blk.svg" alt="Rust Logo" className="w-5 h-5 object-contain invert contrast-200" />
      );
      return <div className="text-xl">💻</div>;
  }

  if (loading) {
    return (
      <div className="min-h-screen bg-background flex items-center justify-center">
        <div className="text-zinc-500 animate-pulse text-sm font-medium tracking-tight">Loading course...</div>
      </div>
    );
  }

  return (
    <div className="min-h-screen bg-background text-foreground selection:bg-primary selection:text-primary-foreground">
      {/* Header */}
      <header className="border-b border-border bg-background/80 backdrop-blur-md sticky top-0 z-50">
        <div className="container mx-auto px-6 h-14 flex items-center justify-between">
          <div className="flex items-center gap-4">
            <Link href="/" className="outline-none focus-visible:ring-2 ring-primary ring-offset-2 ring-offset-background rounded-sm">
              <div className="w-8 h-8 flex items-center justify-center text-lg hover:opacity-80 transition cursor-pointer bg-primary text-primary-foreground rounded">
                {getCourseIcon(courseId)}
              </div>
            </Link>
            <div className="flex items-baseline gap-3">
              <h1 className="text-sm font-semibold tracking-tight capitalize">Learn {courseName.split(" ")[0]}</h1>
              <span className="text-xs text-zinc-500 font-medium tracking-tight border-l border-border pl-3">bootdotdev course</span>
            </div>
          </div>
          <Badge variant="outline" className="text-[10px] text-zinc-500 border-border uppercase tracking-wider px-1.5 rounded-sm">
            {chapters.length} Chapters
          </Badge>
        </div>
      </header>

      {/* Chapters List */}
      <section className="container mx-auto px-6 py-12 pb-24">
        <div className="mb-10">
          <h2 className="text-xs font-bold tracking-[0.2em] uppercase text-zinc-500 mb-2">Course Content</h2>
          <div className="h-[2px] w-12 bg-primary" />
        </div>
        <ScrollArea className="h-full">
          <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
            {chapters.map((chapter) => {
              const exercises = chapterExercises[chapter.id] || [];
              const progress = getChapterProgress(
                courseId,
                chapter.id,
                exercises.map((e) => e.id)
              );

              return (
                <Link key={chapter.id} href={`/course/${courseId}/chapter/${chapter.id}`} className="group outline-none">
                  <Card className="h-full bg-transparent hover:bg-zinc-900/30 border-border group-hover:border-zinc-700 transition-all duration-300 rounded-xl relative overflow-hidden group-focus-visible:ring-2 ring-primary ring-offset-2 ring-offset-background">
                    <div className="absolute inset-0 bg-gradient-to-br from-white/[0.02] to-transparent opacity-0 group-hover:opacity-100 transition-opacity duration-500" />
                    <CardHeader className="pb-4 relative z-10">
                      <div className="flex items-start justify-between mb-2">
                        <div className="text-xl opacity-80 group-hover:opacity-100 filter grayscale group-hover:grayscale-0 transition-all duration-500">
                          {getChapterIcon(chapter.name)}
                        </div>
                        <Badge
                          variant="outline"
                          className="text-[10px] text-zinc-500 tracking-wider uppercase border-border rounded-sm px-1.5 group-hover:border-zinc-700 transition-colors"
                        >
                          {exercises.length} <span className="hidden sm:inline ml-1">exercises</span>
                        </Badge>
                      </div>
                      <CardTitle className="text-base font-semibold tracking-tight text-foreground group-hover:text-primary transition-colors capitalize pt-4 border-b border-border/50 pb-4">
                        <span className="text-zinc-500 mr-2">{chapter.order}.</span> {chapter.name.replace(/_/g, " ")}
                      </CardTitle>
                    </CardHeader>
                    <CardContent className="relative z-10 pt-2 pb-6">
                      <div className="space-y-3 mt-auto">
                        <div className="flex items-center justify-between text-[11px] font-medium tracking-wider uppercase">
                          <span className="text-zinc-500">Progress</span>
                          <span className="text-zinc-400 font-mono">{progress}%</span>
                        </div>
                        <Progress value={progress} className="h-1 bg-zinc-900/50 outline outline-1 outline-offset-[1px] outline-border/50 [&>div]:bg-primary" />
                      </div>
                    </CardContent>
                  </Card>
                </Link>
              );
            })}
          </div>
        </ScrollArea>
      </section>
    </div>
  );
}
