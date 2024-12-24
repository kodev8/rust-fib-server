import { render, screen } from "@testing-library/react";
import { BrowserRouter } from "react-router-dom";
import Video from "./video";

describe("Video Component", () => {
  const renderVideo = () => {
    return render(
      <BrowserRouter>
        <Video />
      </BrowserRouter>
    );
  };

  it("renders video component correctly", () => {
    renderVideo();
    expect(screen.getByText("Video Player")).toBeInTheDocument();
    expect(screen.getByText("Watch our featured content")).toBeInTheDocument();
  });

  it("renders iframe with correct attributes", () => {
    renderVideo();
    const iframe = screen.getByTitle("YouTube video player");
    expect(iframe).toBeInTheDocument();
    expect(iframe).toHaveAttribute("width", "100%");
    expect(iframe).toHaveAttribute("height", "100%");
    expect(iframe).toHaveAttribute("frameBorder", "0");
    expect(iframe).toHaveAttribute("allowFullScreen");
  });

  it("renders navigation link correctly", () => {
    renderVideo();
    const calculatorLink = screen.getByText("Calculator");
    expect(calculatorLink).toBeInTheDocument();
    expect(calculatorLink.closest("a")).toHaveAttribute("href", "/");
  });

  it("applies correct styling classes", () => {
    renderVideo();
    const mainContainer = screen.getByRole("main");
    expect(mainContainer).toHaveClass("min-h-screen", "bg-gradient-to-br");

    const videoContainer = screen.getByTitle(
      "YouTube video player"
    ).parentElement;
    expect(videoContainer).toHaveClass(
      "relative",
      "aspect-video",
      "rounded-xl"
    );
  });
});
