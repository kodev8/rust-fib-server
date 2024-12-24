import "./App.css";
import {
  Route,
  createBrowserRouter,
  createRoutesFromElements,
  RouterProvider,
} from "react-router-dom";
import Calculator from "./pages/Calculator";
import Video from "./pages/video";
import Factorial from "./pages/Factorial";

function App() {
  const router = createBrowserRouter(
    createRoutesFromElements(
      <Route path="/">
        <Route index element={<Calculator />} />
        <Route path="/tony" element={<Video />} />
        <Route path="/factorial" element={<Factorial />} />
      </Route>
    )
  );

  return (
    <>
      <RouterProvider router={router} />
    </>
  );
}

export default App;
