const API_BASE = "http://localhost:3001/api";

export interface Course {
  id: string;
  name: string;
}

export interface Chapter {
  id: string;
  name: string;
  order: number;
}

export interface Exercise {
  id: string;
  name: string;
  type: "code" | "multiple_choice";
  order: number;
}

export interface ExerciseDetail {
  id: string;
  name: string;
  type: "code" | "multiple_choice";
  readme: string;
  starterCode?: string;
  expectedOutput?: string;
  multipleChoice?: {
    question: string;
    answers: string[];
  };
}

export interface ExecuteResult {
  stdout: string;
  stderr: string;
  exitCode: number;
  success: boolean;
  passed: boolean;
}

export async function fetchCourses(): Promise<Course[]> {
  const res = await fetch(`${API_BASE}/courses`);
  if (!res.ok) throw new Error("Failed to fetch courses");
  return res.json();
}

export async function fetchChapters(courseId: string): Promise<Chapter[]> {
  const url = `${API_BASE}/courses/${courseId}/chapters`;
  const res = await fetch(url);
  if (!res.ok) {
    console.error(`fetchChapters failed: ${url} - ${res.status} ${res.statusText}`);
    throw new Error(`Failed to fetch chapters: ${res.status} ${res.statusText}`);
  }
  return res.json();
}

export async function fetchExercises(courseId: string, chapterId: string): Promise<Exercise[]> {
  const url = `${API_BASE}/courses/${courseId}/chapters/${chapterId}/exercises`;
  const res = await fetch(url);
  if (!res.ok) {
    console.error(`fetchExercises failed: ${url} - ${res.status} ${res.statusText}`);
    throw new Error(`Failed to fetch exercises: ${res.status} ${res.statusText}`);
  }
  return res.json();
}

export async function fetchExercise(courseId: string, chapterId: string, exerciseId: string): Promise<ExerciseDetail> {
  const res = await fetch(`${API_BASE}/courses/${courseId}/chapters/${chapterId}/exercises/${exerciseId}`);
  if (!res.ok) throw new Error("Failed to fetch exercise");
  return res.json();
}

export async function fetchSolution(courseId: string, chapterId: string, exerciseId: string): Promise<string> {
  const res = await fetch(`${API_BASE}/courses/${courseId}/chapters/${chapterId}/exercises/${exerciseId}/solution`);
  if (!res.ok) throw new Error("Failed to fetch solution");
  const data = await res.json();
  return data.solution;
}

export async function executeCode(code: string, language: string, expectedOutput?: string): Promise<ExecuteResult> {
  const res = await fetch(`${API_BASE}/execute`, {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify({ code, language, expectedOutput }),
  });
  if (!res.ok) throw new Error("Failed to execute code");
  return res.json();
}
