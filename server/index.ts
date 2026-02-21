import { Hono } from "hono";
import { cors } from "hono/cors";
import { readdir, readFile } from "fs/promises";
import { join } from "path";
import { existsSync } from "fs";

const app = new Hono();

// CORS for frontend
app.use(
  "/*",
  cors({
    origin: ["http://localhost:3000"],
    allowMethods: ["GET", "POST", "OPTIONS"],
    allowHeaders: ["Content-Type"],
  }),
);

const COURSES_PATH = join(import.meta.dir, "..", "courses");

// ... Chapter, Exercise, ExerciseDetail, parseChapterName, parseExerciseName ...
interface Course {
  id: string;
  name: string;
}

interface Chapter {
  id: string;
  name: string;
  order: number;
}

interface Exercise {
  id: string;
  name: string;
  type: "code" | "multiple_choice";
  order: number;
}

interface ExerciseDetail {
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

// Parse chapter directory name to get order and name
function parseChapterName(dirname: string): { order: number; name: string } {
  const match = dirname.match(/^(\d+)-(.+)$/);
  if (match) {
    return {
      order: parseInt(match[1], 10),
      name: match[2].replace(/_/g, " "),
    };
  }
  return { order: 0, name: dirname };
}

// Parse exercise directory name
function parseExerciseName(dirname: string): { order: number; name: string } {
  const match = dirname.match(/^(\d+[a-z]?)-(.+)$/);
  if (match) {
    return {
      order: parseFloat(
        match[1].replace(/[a-z]/, (c) => "." + (c.charCodeAt(0) - 96)),
      ),
      name: match[2].replace(/_/g, " "),
    };
  }
  return { order: 0, name: dirname };
}

// GET /api/courses - List all courses
app.get("/api/courses", async (c) => {
  try {
    const entries = await readdir(COURSES_PATH, { withFileTypes: true });
    const courses: Course[] = entries
      .filter((entry) => entry.isDirectory())
      .map((entry) => {
        return {
          id: entry.name,
          name: entry.name.replace(/-/g, " "),
        };
      });

    return c.json(courses);
  } catch (error) {
    return c.json({ error: "Failed to read courses" }, 500);
  }
});

// GET /api/courses/:courseId/chapters - List all chapters
app.get("/api/courses/:courseId/chapters", async (c) => {
  const { courseId } = c.req.param();
  const coursePath = join(COURSES_PATH, courseId);
  try {
    if (!existsSync(coursePath)) {
      return c.json({ error: "Course not found" }, 404);
    }
    const entries = await readdir(coursePath, { withFileTypes: true });
    const chapters: Chapter[] = entries
      .filter((entry) => entry.isDirectory())
      .map((entry) => {
        const parsed = parseChapterName(entry.name);
        return {
          id: entry.name,
          name: parsed.name,
          order: parsed.order,
        };
      })
      .sort((a, b) => a.order - b.order);

    return c.json(chapters);
  } catch (error) {
    return c.json({ error: "Failed to read chapters" }, 500);
  }
});

// GET /api/courses/:courseId/chapters/:chapterId/exercises - List exercises in a chapter
app.get("/api/courses/:courseId/chapters/:chapterId/exercises", async (c) => {
  const { courseId, chapterId } = c.req.param();
  const exercisesPath = join(COURSES_PATH, courseId, chapterId, "exercises");

  if (!existsSync(exercisesPath)) {
    return c.json({ error: "Course or chapter not found" }, 404);
  }

  try {
    const entries = await readdir(exercisesPath, { withFileTypes: true });
    const exercises: Exercise[] = await Promise.all(
      entries
        .filter((entry) => entry.isDirectory())
        .map(async (entry) => {
          const parsed = parseExerciseName(entry.name);
          const mcPath = join(
            exercisesPath,
            entry.name,
            "multiple_choice.json",
          );
          const hasMultipleChoice = existsSync(mcPath);

          return {
            id: entry.name,
            name: parsed.name,
            type: hasMultipleChoice
              ? ("multiple_choice" as const)
              : ("code" as const),
            order: parsed.order,
          };
        }),
    );

    exercises.sort((a, b) => a.order - b.order);
    return c.json(exercises);
  } catch (error) {
    return c.json({ error: "Failed to read exercises" }, 500);
  }
});

// GET /api/courses/:courseId/chapters/:chapterId/exercises/:exerciseId - Get exercise details
app.get(
  "/api/courses/:courseId/chapters/:chapterId/exercises/:exerciseId",
  async (c) => {
    const { courseId, chapterId, exerciseId } = c.req.param();
    const exercisePath = join(
      COURSES_PATH,
      courseId,
      chapterId,
      "exercises",
      exerciseId,
    );

    try {
      if (!existsSync(exercisePath)) {
        return c.json({ error: "Exercise not found" }, 404);
      }

      const parsed = parseExerciseName(exerciseId);
      const mcPath = join(exercisePath, "multiple_choice.json");
      const hasMultipleChoice = existsSync(mcPath);

      const readmeContent = await readFile(
        join(exercisePath, "readme.md"),
        "utf-8",
      );

      const exercise: ExerciseDetail = {
        id: exerciseId,
        name: parsed.name,
        type: hasMultipleChoice ? "multiple_choice" : "code",
        readme: readmeContent,
      };

      if (hasMultipleChoice) {
        const mcContent = await readFile(mcPath, "utf-8");
        exercise.multipleChoice = JSON.parse(mcContent);
      } else {
        // Code exercise
        const codePathGo = join(exercisePath, "code.go");
        const codePathRust = join(exercisePath, "code.rs");
        const expectedPath = join(exercisePath, "expected.txt");

        if (existsSync(codePathGo)) {
          exercise.starterCode = await readFile(codePathGo, "utf-8");
        } else if (existsSync(codePathRust)) {
          exercise.starterCode = await readFile(codePathRust, "utf-8");
        }

        if (existsSync(expectedPath)) {
          exercise.expectedOutput = await readFile(expectedPath, "utf-8");
        }
      }

      return c.json(exercise);
    } catch (error) {
      return c.json({ error: "Failed to read exercise" }, 500);
    }
  },
);

// GET /api/courses/:courseId/chapters/:chapterId/exercises/:exerciseId/solution - Get solution
app.get(
  "/api/courses/:courseId/chapters/:chapterId/exercises/:exerciseId/solution",
  async (c) => {
    const { courseId, chapterId, exerciseId } = c.req.param();
    const basePath = join(
      COURSES_PATH,
      courseId,
      chapterId,
      "exercises",
      exerciseId,
    );
    const solutionPathGo = join(basePath, "complete.go");
    const solutionPathRust = join(basePath, "complete.rs");

    try {
      if (existsSync(solutionPathGo)) {
        const solution = await readFile(solutionPathGo, "utf-8");
        return c.json({ solution });
      } else if (existsSync(solutionPathRust)) {
        const solution = await readFile(solutionPathRust, "utf-8");
        return c.json({ solution });
      } else {
        return c.json({ error: "No solution available" }, 404);
      }
    } catch (error) {
      return c.json({ error: "Failed to read solution" }, 500);
    }
  },
);

// POST /api/execute - Execute Code locally
app.post("/api/execute", async (c) => {
  const body = await c.req.json<{ code: string; language: string; expectedOutput?: string }>();
  const { code, language, expectedOutput } = body;

  if (!code || !language) {
    return c.json({ error: "Code and language are required" }, 400);
  }

  try {
    const tmpDir = "/tmp/golearn";
    if (!existsSync(tmpDir)) {
      await Bun.write(tmpDir + "/.keep", "");
    }
    const timestamp = Date.now();
    let proc;
    let tmpFile = "";
    let tmpBin = "";

    if (language === "go") {
      tmpFile = join(tmpDir, `main_${timestamp}.go`);
      await Bun.write(tmpFile, code);
      proc = Bun.spawn(["go", "run", tmpFile], {
        stdout: "pipe",
        stderr: "pipe",
      });
    } else if (language === "rust") {
      tmpFile = join(tmpDir, `main_${timestamp}.rs`);
      tmpBin = join(tmpDir, `bin_${timestamp}`);
      await Bun.write(tmpFile, code);
      
      // Compile Rust
      const compileProc = Bun.spawn(["rustc", tmpFile, "-o", tmpBin], {
         stdout: "pipe",
         stderr: "pipe",
      });
      const compileExitCode = await compileProc.exited;
      if (compileExitCode !== 0) {
        const compileStderr = await new Response(compileProc.stderr).text();
        return c.json({
          stdout: "",
          stderr: compileStderr.trim(),
          exitCode: compileExitCode,
          success: false,
          passed: false,
        });
      }

      // Execute Bin
      proc = Bun.spawn([tmpBin], {
         stdout: "pipe",
         stderr: "pipe",
      });
    } else {
       return c.json({ error: "Unsupported language" }, 400);
    }

    const timeout = setTimeout(() => {
      proc?.kill();
    }, 10000); // 10 second timeout

    const stdout = await new Response(proc.stdout).text();
    const stderr = await new Response(proc.stderr).text();
    const exitCode = await proc.exited;

    clearTimeout(timeout);

    // Clean up
    if (tmpFile) await Bun.write(tmpFile, ""); // Clear source file (or could delete)
    if (tmpBin && existsSync(tmpBin)) Bun.spawn(["rm", tmpBin]); // Clean up rust binary

    const result = {
      stdout: stdout.trim(),
      stderr: stderr.trim(),
      exitCode,
      success: exitCode === 0,
      passed: false,
    };

    // Validate against expected output if provided
    if (expectedOutput && result.success) {
      result.passed = stdout.trim() === expectedOutput.trim();
    }

    return c.json(result);
  } catch (error) {
    return c.json(
      {
        error: "Failed to execute code",
        details: error instanceof Error ? error.message : "Unknown error",
      },
      500
    );
  }
});

// Ensure tmp directory exists at startup
const tmpDir = "/tmp/golearn";
if (!existsSync(tmpDir)) {
  await Bun.write(tmpDir + "/.keep", "");
}

const port = 3001;
console.log(`🚀 Server running at http://localhost:${port}`);

export default {
  port,
  fetch: app.fetch,
};
