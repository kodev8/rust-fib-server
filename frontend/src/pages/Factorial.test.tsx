import { render, screen, fireEvent, waitFor } from "@testing-library/react";
import { BrowserRouter } from "react-router-dom";
import Factorial from "./Factorial";

// Mock fetch globally
const mockFetch = jest.fn();
global.fetch = mockFetch;

describe("Factorial Component", () => {
  beforeEach(() => {
    mockFetch.mockClear();
  });

  const renderFactorial = () => {
    return render(
      <BrowserRouter>
        <Factorial />
      </BrowserRouter>
    );
  };

  it("renders factorial component correctly", () => {
    renderFactorial();
    expect(screen.getByText("Factorial Calculator")).toBeInTheDocument();
    expect(
      screen.getByPlaceholderText("Enter a non-negative number")
    ).toBeInTheDocument();
    expect(screen.getByText("Calculate Factorial")).toBeInTheDocument();
  });

  it("handles invalid input correctly", async () => {
    renderFactorial();
    const input = screen.getByPlaceholderText("Enter a non-negative number");
    const button = screen.getByText("Calculate Factorial");

    // Test negative number
    fireEvent.change(input, { target: { value: "-1" } });
    fireEvent.click(button);
    expect(
      screen.getByText("Please enter a valid non-negative number")
    ).toBeInTheDocument();

    // Test empty input
    fireEvent.change(input, { target: { value: "" } });
    fireEvent.click(button);
    expect(
      screen.getByText("Please enter a valid non-negative number")
    ).toBeInTheDocument();

    // Test non-numeric input
    fireEvent.change(input, { target: { value: "abc" } });
    fireEvent.click(button);
    expect(
      screen.getByText("Please enter a valid non-negative number")
    ).toBeInTheDocument();
  });

  it("calculates factorial number successfully", async () => {
    const mockResponse = { message: "Success", result: "120", cached: false };
    mockFetch.mockImplementationOnce(() =>
      Promise.resolve({
        ok: true,
        json: () => Promise.resolve(mockResponse),
      })
    );

    renderFactorial();
    const input = screen.getByPlaceholderText("Enter a non-negative number");
    const button = screen.getByText("Calculate Factorial");

    fireEvent.change(input, { target: { value: "5" } });
    fireEvent.click(button);

    await waitFor(() => {
      expect(screen.getByText("Success")).toBeInTheDocument();
      expect(screen.getByText("120")).toBeInTheDocument();
      expect(screen.getByText("Freshly calculated")).toBeInTheDocument();
    });

    expect(mockFetch).toHaveBeenCalledWith(
      "http://localhost:8000/factorial?num=5",
      expect.any(Object)
    );
  });

  it("handles API error correctly", async () => {
    mockFetch.mockImplementationOnce(() =>
      Promise.reject(new Error("API Error"))
    );

    renderFactorial();
    const input = screen.getByPlaceholderText("Enter a non-negative number");
    const button = screen.getByText("Calculate Factorial");

    fireEvent.change(input, { target: { value: "5" } });
    fireEvent.click(button);

    await waitFor(() => {
      expect(
        screen.getByText(
          "Failed to fetch result. Make sure the backend server is running."
        )
      ).toBeInTheDocument();
    });
  });

  it("shows loading state while calculating", async () => {
    mockFetch.mockImplementationOnce(
      () =>
        new Promise((resolve) =>
          setTimeout(
            () =>
              resolve({
                ok: true,
                json: () =>
                  Promise.resolve({
                    message: "Success",
                    result: "120",
                    cached: true,
                  }),
              }),
            100
          )
        )
    );

    renderFactorial();
    const input = screen.getByPlaceholderText("Enter a non-negative number");
    const button = screen.getByText("Calculate Factorial");

    fireEvent.change(input, { target: { value: "5" } });
    fireEvent.click(button);

    expect(screen.getByText("Calculating...")).toBeInTheDocument();

    await waitFor(() => {
      expect(screen.getByText("Success")).toBeInTheDocument();
      expect(screen.getByText("Result from cache")).toBeInTheDocument();
    });
  });

  it("renders navigation links correctly", () => {
    renderFactorial();
    const videoLink = screen.getByText("Video");
    const homeLink = screen.getByText("Home");

    expect(videoLink).toBeInTheDocument();
    expect(videoLink.closest("a")).toHaveAttribute("href", "/tony");

    expect(homeLink).toBeInTheDocument();
    expect(homeLink.closest("a")).toHaveAttribute("href", "/");
  });
});
