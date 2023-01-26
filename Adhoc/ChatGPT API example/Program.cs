using System.Text;
using Newtonsoft.Json;

if (args.Length > 0)
{
    /* Improvement - Learn about Http Factory (For a bigger application)*/
    HttpClient client = new HttpClient();

    client.DefaultRequestHeaders.Add("authorization","Bearer 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"); //Never Paste Key to the Code (User Secrets) 

    /* Create object and Serialize the request 
    Learn what are the differences between the models*/
    var content = new StringContent("{\"model\": \"text-davinci-001\", \"prompt\": \""+ args[0] +"\",\"temperature\": 1,\"max_tokens\": 100}",
        Encoding.UTF8, "application/json");

    HttpResponseMessage response = await client.PostAsync("https://api.openai.com/v1/completions", content);
    
    string responseString = await response.Content.ReadAsStringAsync();

    Console.WriteLine(responseString);
    try 
    {
        var dyData = JsonConvert.DeserializeObject<dynamic>(responseString); /*Instead of using dynamic type object Create a object and desseialize that using that object*/

        Console.ForegroundColor = ConsoleColor.Green;
        Console.WriteLine($"---> ChatGpt Response : {dyData!.choices[0].text}");
        Console.ResetColor();

    }
    catch(Exception e)
    {
        Console.ForegroundColor = ConsoleColor.Red;
        Console.WriteLine(e.Message);
        Console.ResetColor();
    }
}
else 
{
    Console.WriteLine("Please provide at least one input");
}