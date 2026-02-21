import { create } from "zustand";
import { persist } from "zustand/middleware";

interface ProgressState {
  completedExercises: Set<string>;
  markComplete: (courseId: string, chapterId: string, exerciseId: string) => void;
  isComplete: (courseId: string, chapterId: string, exerciseId: string) => boolean;
  getChapterProgress: (courseId: string, chapterId: string, exerciseIds: string[]) => number;
}

// Create a unique key for each exercise scoped by course
const getKey = (courseId: string, chapterId: string, exerciseId: string) => `${courseId}/${chapterId}/${exerciseId}`;

export const useProgressStore = create<ProgressState>()(
  persist(
    (set, get) => ({
      completedExercises: new Set<string>(),

      markComplete: (courseId: string, chapterId: string, exerciseId: string) => {
        const key = getKey(courseId, chapterId, exerciseId);
        set((state) => ({
          completedExercises: new Set(state.completedExercises).add(key),
        }));
      },

      isComplete: (courseId: string, chapterId: string, exerciseId: string) => {
        const key = getKey(courseId, chapterId, exerciseId);
        return get().completedExercises.has(key);
      },

      getChapterProgress: (courseId: string, chapterId: string, exerciseIds: string[]) => {
        if (exerciseIds.length === 0) return 0;
        const completed = exerciseIds.filter((id) =>
          get().isComplete(courseId, chapterId, id)
        ).length;
        return Math.round((completed / exerciseIds.length) * 100);
      },
    }),
    {
      name: "learn-code-progress",
      // Custom serializer for Set
      storage: {
        getItem: (name) => {
          const str = localStorage.getItem(name);
          if (!str) return null;
          const data = JSON.parse(str);
          return {
            ...data,
            state: {
              ...data.state,
              completedExercises: new Set(data.state.completedExercises || []),
            },
          };
        },
        setItem: (name, value) => {
          const data = {
            ...value,
            state: {
              ...value.state,
              completedExercises: Array.from(value.state.completedExercises),
            },
          };
          localStorage.setItem(name, JSON.stringify(data));
        },
        removeItem: (name) => localStorage.removeItem(name),
      },
    }
  )
);
