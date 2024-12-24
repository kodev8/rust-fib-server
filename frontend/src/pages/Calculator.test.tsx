import { render, screen, fireEvent, waitFor } from "@testing-library/react";
import { BrowserRouter } from "react-router-dom";
import Calculator from "./Calculator";

// Mock fetch globally
const mockFetch = jest.fn();
global.fetch = mockFetch;

describe("Calculator Component", () => {
  beforeEach(() => {
    mockFetch.mockClear();
  });

  const renderCalculator = () => {
    return render(
      <BrowserRouter>
        <Calculator />
      </BrowserRouter>
    );
  };

  it("renders calculator component correctly", () => {
    renderCalculator();
    expect(screen.getByText("Fibonacci Calculator")).toBeInTheDocument();
    expect(
      screen.getByPlaceholderText("Enter a non-negative number")
    ).toBeInTheDocument();
    expect(screen.getByText("Calculate Fibonacci")).toBeInTheDocument();
  });

  it("handles invalid input correctly", async () => {
    renderCalculator();
    const input = screen.getByPlaceholderText("Enter a non-negative number");
    const button = screen.getByText("Calculate Fibonacci");

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

  it("calculates fibonacci number successfully", async () => {
    const mockResponse = { message: "Success", fib: 5 };
    mockFetch.mockImplementationOnce(() =>
      Promise.resolve({
        ok: true,
        json: () => Promise.resolve(mockResponse),
      })
    );

    renderCalculator();
    const input = screen.getByPlaceholderText("Enter a non-negative number");
    const button = screen.getByText("Calculate Fibonacci");

    fireEvent.change(input, { target: { value: "5" } });
    fireEvent.click(button);

    await waitFor(() => {
      expect(screen.getByText("Success")).toBeInTheDocument();
      expect(screen.getByText("5")).toBeInTheDocument();
    });

    expect(mockFetch).toHaveBeenCalledWith(
      "http://localhost:8080/fib?num=5",
      expect.any(Object)
    );
  });

  it("handles API error correctly", async () => {
    mockFetch.mockImplementationOnce(() =>
      Promise.reject(new Error("API Error"))
    );

    renderCalculator();
    const input = screen.getByPlaceholderText("Enter a non-negative number");
    const button = screen.getByText("Calculate Fibonacci");

    fireEvent.change(input, { target: { value: "5" } });
    fireEvent.click(button);

    await waitFor(() => {
      expect(
        screen.getByText(
          "Failed to fetch result. Make sure the backend server is running."
        )
      ).toBeInTheDocument();
    });

    expect(mockFetch).toHaveBeenCalledWith(
      "http://localhost:8080/fib?num=5",
      expect.any(Object)
    );
  });

  it("shows loading state while calculating", async () => {
    mockFetch.mockImplementationOnce(
      () =>
        new Promise((resolve) =>
          setTimeout(
            () =>
              resolve({
                ok: true,
                json: () => Promise.resolve({ message: "Success", fib: 5 }),
              }),
            100
          )
        )
    );

    renderCalculator();
    const input = screen.getByPlaceholderText("Enter a non-negative number");
    const button = screen.getByText("Calculate Fibonacci");

    fireEvent.change(input, { target: { value: "5" } });
    fireEvent.click(button);

    expect(screen.getByText("Calculating...")).toBeInTheDocument();

    await waitFor(() => {
      expect(screen.getByText("Success")).toBeInTheDocument();
    });
  });

  it("renders navigation link correctly", () => {
    renderCalculator();
    const homeLink = screen.getByText("Home");
    expect(homeLink).toBeInTheDocument();
    expect(homeLink.closest("a")).toHaveAttribute("href", "/");
  });
});
