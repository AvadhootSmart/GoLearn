"use client";

import { useEffect, useState } from "react";
import Link from "next/link";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { fetchCourses, Course } from "@/lib/api";
import { BookOpen } from "lucide-react";

export default function CoursesPage() {
  const [courses, setCourses] = useState<Course[]>([]);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    async function loadData() {
      try {
        const coursesData = await fetchCourses();
        setCourses(coursesData);
      } catch (error) {
        console.error("Failed to load courses:", error);
      } finally {
        setLoading(false);
      }
    }
    loadData();
  }, []);

  const getCourseIcon = (id: string): React.ReactNode => {
      if (id.includes("go")) return (
        <img src="https://go.dev/blog/go-brand/Go-Logo/PNG/Go-Logo_Blue.png" alt="Go Logo" className="w-10 h-10 object-contain p-1" />
      );
      if (id.includes("rust")) return (
        <img src="https://rust-lang.org/logos/rust-logo-blk.svg" alt="Rust Logo" className="w-10 h-10 object-contain p-1 invert contrast-200" />
      );
      return <div className="text-3xl">💻</div>;
  }

  if (loading) {
    return (
      <div className="min-h-screen bg-background flex items-center justify-center">
        <div className="text-zinc-500 animate-pulse text-sm font-medium tracking-tight">Loading courses...</div>
      </div>
    );
  }

  return (
    <div className="min-h-screen bg-background text-foreground selection:bg-primary selection:text-primary-foreground">
      {/* Header */}
      <header className="border-b border-border bg-background/80 backdrop-blur-md sticky top-0 z-50">
        <div className="container mx-auto px-6 h-14 flex items-center justify-between">
          <div className="flex items-center gap-3">
            <h1 className="text-xl font-bold tracking-tight">Go Learn</h1>
          </div>
        </div>
      </header>

      {/* Hero Section */}
      <section className="container mx-auto px-6 py-20 pb-16">
        <div className="max-w-2xl">
          <h2 className="text-4xl md:text-5xl font-bold tracking-tighter text-balance mb-6 uppercase">
            Master Backend<br />
            <span className="text-zinc-500">Programming.</span>
          </h2>
          <p className="text-lg text-zinc-400 mb-8 leading-relaxed max-w-xl">
            Interactive courses with hands-on coding exercises. Get instant feedback and track your progress natively.
          </p>
          <div className="flex flex-col sm:flex-row sm:items-center gap-4 text-sm text-zinc-500 font-medium tracking-tight">
            <div className="flex items-center gap-2">
              <span className="text-primary">✓</span> Vim keybindings
            </div>
            <div className="flex items-center gap-2">
              <span className="text-primary">✓</span> Local execution
            </div>
            <div className="flex items-center gap-2">
              <span className="text-primary">✓</span> Progress tracking
            </div>
          </div>
        </div>
      </section>

      {/* Courses Grid */}
      <section className="container mx-auto px-6 pb-24">
        <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
          {courses.map((course) => (
            <Link key={course.id} href={`/course/${course.id}`} className="group outline-none">
              <Card className="h-full bg-transparent hover:bg-zinc-900/30 border-border group-hover:border-zinc-700 transition-all duration-300 rounded-xl overflow-hidden relative cursor-pointer group-focus-visible:ring-2 ring-primary ring-offset-2 ring-offset-background">
                <div className="absolute inset-0 bg-gradient-to-br from-white/[0.02] to-transparent opacity-0 group-hover:opacity-100 transition-opacity duration-500" />
                <CardHeader className="relative z-10">
                  <div className="flex items-center justify-between mb-8">
                    <div className="w-12 h-12 flex items-center justify-center p-2 rounded-lg bg-zinc-900/50 border border-white/[0.05] filter grayscale group-hover:grayscale-0 transition-all duration-500">
                      {getCourseIcon(course.id)}
                    </div>
                    <div className="w-8 h-8 rounded-full border border-border flex items-center justify-center opacity-0 -translate-x-2 group-hover:opacity-100 group-hover:translate-x-0 transition-all duration-300">
                      <span className="text-zinc-500 text-sm">→</span>
                    </div>
                  </div>
                  <CardTitle className="text-xl font-bold tracking-tight capitalize group-hover:text-primary transition-colors">
                    {course.name}
                  </CardTitle>
                </CardHeader>
                <CardContent className="relative z-10">
                  <p className="text-zinc-500 text-sm leading-relaxed">
                    Interactive path to learning {course.name} fundamentals and building robust backends.
                  </p>
                </CardContent>
              </Card>
            </Link>
          ))}
        </div>
      </section>
    </div>
  );
}
