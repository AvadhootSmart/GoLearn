"use client";

import { useState } from "react";
import { Button } from "@/components/ui/button";
import { Card } from "@/components/ui/card";
import { cn } from "@/lib/utils";

interface MultipleChoiceProps {
  question: string;
  answers: string[];
  onAnswer: (correct: boolean) => void;
}

export function MultipleChoice({ question, answers, onAnswer }: MultipleChoiceProps) {
  const [selectedIndex, setSelectedIndex] = useState<number | null>(null);
  const [submitted, setSubmitted] = useState(false);
  const [isCorrect, setIsCorrect] = useState(false);

  // First answer is always the correct one in the course data
  const correctIndex = 0;

  const handleSubmit = () => {
    if (selectedIndex === null) return;
    const correct = selectedIndex === correctIndex;
    setIsCorrect(correct);
    setSubmitted(true);
    onAnswer(correct);
  };

  const handleReset = () => {
    setSelectedIndex(null);
    setSubmitted(false);
    setIsCorrect(false);
  };

  return (
    <div className="p-8 max-w-2xl mx-auto space-y-8">
      <h3 className="text-xl font-semibold tracking-tight text-foreground leading-snug">{question}</h3>
      
      <div className="space-y-3">
        {answers.map((answer, index) => (
          <Card
            key={index}
            className={cn(
              "p-4 cursor-pointer transition-all border outline-none",
              selectedIndex === index
                ? "border-primary bg-primary/5"
                : "border-border bg-transparent hover:border-zinc-500 hover:bg-zinc-900/30",
              submitted && index === correctIndex && "border-primary bg-primary/5",
              submitted && selectedIndex === index && index !== correctIndex && "border-destructive bg-destructive/5"
            )}
            onClick={() => !submitted && setSelectedIndex(index)}
          >
            <div className="flex items-center gap-4">
              <div
                className={cn(
                  "w-5 h-5 rounded-full border-2 flex items-center justify-center transition-colors",
                  selectedIndex === index || (submitted && index === correctIndex)
                    ? "border-primary"
                    : "border-zinc-600"
                )}
              >
                {(selectedIndex === index || (submitted && index === correctIndex)) && (
                  <div className="w-2.5 h-2.5 rounded-full bg-primary" />
                )}
              </div>
              <span className="text-foreground leading-relaxed text-sm">{answer}</span>
            </div>
          </Card>
        ))}
      </div>

      <div className="flex items-center gap-4 pt-4 border-t border-border mt-8">
        {!submitted ? (
          <Button
            onClick={handleSubmit}
            disabled={selectedIndex === null}
            variant="default"
          >
            Submit Answer
          </Button>
        ) : (
          <>
            <div
              className={cn(
                "px-3 py-1.5 rounded text-sm font-medium border",
                isCorrect
                  ? "border-primary text-primary bg-primary/5"
                  : "border-destructive text-destructive bg-destructive/5"
              )}
            >
              {isCorrect ? "Correct" : "Incorrect"}
            </div>
            {!isCorrect && (
              <Button variant="outline" onClick={handleReset} className="border-border text-zinc-500 hover:text-foreground">
                Try Again
              </Button>
            )}
          </>
        )}
      </div>
    </div>
  );
}
