import java.io.IOException;
import java.net.InetSocketAddress;
import java.util.concurrent.Executors;
import com.sun.net.httpserver.HttpServer;

public class LoomWebServer {
    public static void main(String[] args) throws IOException {
        // Create a simple HTTP server bound to localhost on port 8080
        InetSocketAddress address = new InetSocketAddress("localhost", 8080);
        HttpServer server = HttpServer.create(address, 0);

        // Create a virtual thread executor
        var executor = Executors.newVirtualThreadPerTaskExecutor();

        // Define a handler for the root path
        server.createContext("/", exchange -> {
            String response = "Hello, World!";
            exchange.sendResponseHeaders(200, response.getBytes().length);
            try (var os = exchange.getResponseBody()) {
                os.write(response.getBytes());
            }
        });

        // Set the executor to handle requests with virtual threads
        server.setExecutor(executor);
        
        // Start the server
        server.start();
        System.out.println("Server is listening on http://localhost:8080");
    }
}

