using System;
using System.Collections.Generic;
using System.Diagnostics;
using System.Drawing;
using System.IO;
using System.Text;
using System.Threading.Tasks;
using System.Web.Script.Serialization;
using System.Windows.Forms;

internal static class Program
{
    [STAThread]
    private static int Main(string[] args)
    {
        Application.EnableVisualStyles();
        Application.SetCompatibleTextRenderingDefault(false);

        LauncherApp app = new LauncherApp(Application.ExecutablePath);
        string screenshotPath = GetArgumentValue(args, "-ScreenshotPath");
        if (HasArgument(args, "-SelfTest"))
        {
            app.ShowSelfTest();
            return 0;
        }
        if (!String.IsNullOrWhiteSpace(screenshotPath))
        {
            app.SaveScreenshot(screenshotPath);
            return 0;
        }
        app.Run();
        return 0;
    }

    private static bool HasArgument(string[] args, string name)
    {
        foreach (string arg in args)
        {
            if (String.Equals(arg, name, StringComparison.OrdinalIgnoreCase))
            {
                return true;
            }
        }
        return false;
    }

    private static string GetArgumentValue(string[] args, string name)
    {
        for (int index = 0; index < args.Length - 1; index += 1)
        {
            if (String.Equals(args[index], name, StringComparison.OrdinalIgnoreCase))
            {
                return args[index + 1];
            }
        }
        return "";
    }
}

internal sealed class LauncherApp
{
    private readonly string appRoot;
    private readonly string repoRoot;
    private readonly string settingsPath;
    private readonly JavaScriptSerializer json;

    private string eplusRsExe;
    private string inputPath;
    private string weatherPath;
    private string outputDir;
    private string oracleRoot;
    private string mode;
    private string partialPolicy;
    private string outputFormat;
    private string traceLevel;
    private bool failOnWarning;
    private bool oracleBaseline;
    private bool compareOracle;
    private bool overwrite;
    private bool refreshingUi;
    private bool cancelRequested;

    private Process currentProcess;
    private Task<string> stdoutTask;
    private Task<string> stderrTask;

    private Form form;
    private Label statusLabel;
    private Label stateDetailLabel;
    private TextBox inputBox;
    private TextBox weatherBox;
    private TextBox outputBox;
    private TextBox oracleBox;
    private TextBox exeBox;
    private ComboBox modeCombo;
    private ComboBox partialCombo;
    private ComboBox formatCombo;
    private ComboBox traceCombo;
    private Button runButton;
    private Button cancelButton;
    private Button inputButton;
    private Button weatherButton;
    private Button outputButton;
    private Button oracleButton;
    private Button exeButton;
    private Button failOnWarningButton;
    private Button oracleBaselineButton;
    private Button compareButton;
    private Button overwriteButton;
    private Button openOutputButton;
    private Button openRunReportButton;
    private Button openDiagnosticsButton;
    private Button openSupportReportButton;
    private Button openCompareButton;
    private Button openEvidenceButton;
    private ListBox phaseList;
    private ListBox diagnosticsList;
    private TextBox supportTextBox;
    private TextBox claimBoundaryTextBox;
    private TextBox resultsTextBox;
    private TextBox compareTextBox;
    private TextBox plotsTextBox;
    private TextBox evidenceTextBox;
    private TextBox logsTextBox;
    private Timer timer;

    public LauncherApp(string executablePath)
    {
        appRoot = Path.GetDirectoryName(executablePath);
        if (String.IsNullOrWhiteSpace(appRoot))
        {
            appRoot = Environment.CurrentDirectory;
        }
        appRoot = Path.GetFullPath(appRoot);
        repoRoot = ResolveRepoRoot(appRoot);
        settingsPath = Path.Combine(
            Environment.GetFolderPath(Environment.SpecialFolder.ApplicationData),
            "RustedEnergyPlus",
            "launcher-settings.json");
        json = new JavaScriptSerializer();

        LauncherDefaults defaults = GetLauncherDefaultPaths();
        eplusRsExe = ResolveEplusRsExe();
        inputPath = defaults.Idf ?? "";
        weatherPath = defaults.Weather ?? "";
        outputDir = Path.Combine(appRoot, ".runtime", "ep-launch-output");
        oracleRoot = defaults.OracleRoot ?? "";
        mode = "compatibility";
        partialPolicy = "deny";
        outputFormat = "rust-native";
        traceLevel = "normal";
        failOnWarning = false;
        oracleBaseline = false;
        compareOracle = TestOracleRoot(oracleRoot);
        overwrite = true;
        LoadSettings();
        BuildForm();
        RefreshUi();
    }

    public void Run()
    {
        Application.Run(form);
    }

    public void SaveScreenshot(string screenshotPath)
    {
        string directory = Path.GetDirectoryName(screenshotPath);
        if (!String.IsNullOrWhiteSpace(directory))
        {
            Directory.CreateDirectory(directory);
        }
        form.CreateControl();
        using (Bitmap bitmap = new Bitmap(form.Width, form.Height))
        {
            Rectangle bounds = new Rectangle(0, 0, form.Width, form.Height);
            form.DrawToBitmap(bitmap, bounds);
            bitmap.Save(screenshotPath, System.Drawing.Imaging.ImageFormat.Png);
        }
        form.Dispose();
    }

    public void ShowSelfTest()
    {
        string message = "Direct WinForms launcher self-test passed.\n\n" +
            "app_root=" + appRoot + "\n" +
            "eplus_rs=" + NullToEmpty(eplusRsExe) + "\n" +
            "oracle_root=" + NullToEmpty(oracleRoot) + "\n" +
            "oracle_ready=" + TestOracleRoot(oracleRoot).ToString().ToLowerInvariant() + "\n" +
            "uses_script_host=false";
        MessageBox.Show(message, "Rusted EnergyPlus Launch", MessageBoxButtons.OK, MessageBoxIcon.Information);
    }

    private void BuildForm()
    {
        form = new Form();
        form.Text = "Rusted EnergyPlus Launch";
        form.StartPosition = FormStartPosition.CenterScreen;
        form.Size = new Size(880, 700);
        form.MinimumSize = new Size(880, 700);

        statusLabel = new Label();
        statusLabel.Text = "Ready.";
        statusLabel.Location = new Point(18, 18);
        statusLabel.Size = new Size(830, 24);
        statusLabel.Font = new Font(statusLabel.Font, FontStyle.Bold);
        form.Controls.Add(statusLabel);

        stateDetailLabel = new Label();
        stateDetailLabel.Text = "conformance_claim=false for launcher and arbitrary runs.";
        stateDetailLabel.Location = new Point(18, 42);
        stateDetailLabel.Size = new Size(830, 36);
        stateDetailLabel.ForeColor = Color.DimGray;
        form.Controls.Add(stateDetailLabel);

        inputButton = NewButton("IDF / epJSON", 18, 88, 140, 30);
        inputBox = NewPathBox(92);
        form.Controls.Add(inputButton);
        form.Controls.Add(inputBox);

        weatherButton = NewButton("Weather EPW", 18, 128, 140, 30);
        weatherBox = NewPathBox(132);
        form.Controls.Add(weatherButton);
        form.Controls.Add(weatherBox);

        outputButton = NewButton("Output Folder", 18, 168, 140, 30);
        outputBox = NewPathBox(172);
        form.Controls.Add(outputButton);
        form.Controls.Add(outputBox);

        oracleButton = NewButton("Oracle Folder", 18, 208, 140, 30);
        oracleBox = NewPathBox(212);
        form.Controls.Add(oracleButton);
        form.Controls.Add(oracleBox);

        exeButton = NewButton("CLI Binary", 18, 248, 140, 30);
        exeBox = NewPathBox(252);
        form.Controls.Add(exeButton);
        form.Controls.Add(exeBox);

        form.Controls.Add(NewLabel("Mode", 18, 300, 50, 26));
        modeCombo = NewComboBox(new string[] { "compatibility", "diagnostic", "fast", "experimental" }, mode, 70, 300, 132, 26);
        form.Controls.Add(modeCombo);
        form.Controls.Add(NewLabel("Partial", 214, 300, 52, 26));
        partialCombo = NewComboBox(new string[] { "deny", "allow" }, partialPolicy, 266, 300, 90, 26);
        form.Controls.Add(partialCombo);
        form.Controls.Add(NewLabel("Format", 370, 300, 56, 26));
        formatCombo = NewComboBox(new string[] { "rust-native", "both" }, outputFormat, 428, 300, 120, 26);
        form.Controls.Add(formatCombo);
        form.Controls.Add(NewLabel("Trace", 562, 300, 46, 26));
        traceCombo = NewComboBox(new string[] { "normal", "detailed", "debug" }, traceLevel, 610, 300, 84, 26);
        form.Controls.Add(traceCombo);
        failOnWarningButton = NewButton("Strict Warnings: OFF", 706, 298, 122, 30);
        form.Controls.Add(failOnWarningButton);

        oracleBaselineButton = NewButton("Oracle Baseline: OFF", 18, 342, 170, 34);
        compareButton = NewButton("Oracle Compare: ON", 198, 342, 160, 34);
        overwriteButton = NewButton("Overwrite: ON", 368, 342, 140, 34);
        runButton = NewButton("Run", 520, 342, 90, 34);
        cancelButton = NewButton("Cancel", 620, 342, 90, 34);
        openOutputButton = NewButton("Open Output", 720, 342, 108, 34);
        form.Controls.AddRange(new Control[] {
            oracleBaselineButton, compareButton, overwriteButton, runButton, cancelButton, openOutputButton
        });

        openRunReportButton = NewButton("Open Run Report", 18, 390, 150, 34);
        openDiagnosticsButton = NewButton("Open Diagnostics", 180, 390, 150, 34);
        openSupportReportButton = NewButton("Open Support Report", 342, 390, 166, 34);
        openCompareButton = NewButton("Open Compare Report", 520, 390, 160, 34);
        openEvidenceButton = NewButton("Open Evidence", 692, 390, 136, 34);
        form.Controls.AddRange(new Control[] {
            openRunReportButton, openDiagnosticsButton, openSupportReportButton, openCompareButton, openEvidenceButton
        });

        TabControl resultTabs = new TabControl();
        resultTabs.Location = new Point(18, 432);
        resultTabs.Size = new Size(810, 188);
        resultTabs.Anchor = AnchorStyles.Left | AnchorStyles.Right | AnchorStyles.Bottom | AnchorStyles.Top;

        phaseList = NewListBox("No phase timing.");
        diagnosticsList = NewListBox("No diagnostics.");
        supportTextBox = NewReadOnlyMultilineBox("Support report will appear after a run.");
        claimBoundaryTextBox = NewReadOnlyMultilineBox("Claim boundary will appear after a run.");
        resultsTextBox = NewReadOnlyMultilineBox("Result artifacts will appear after a supported Rust run.");
        compareTextBox = NewReadOnlyMultilineBox("Oracle comparison artifacts will appear when compare is enabled.");
        plotsTextBox = NewReadOnlyMultilineBox("Plot artifacts will appear after a run writes reports\\plots, plots, or compare\\plots.");
        evidenceTextBox = NewReadOnlyMultilineBox("Evidence summary/PDF artifacts will appear after a run writes reports\\evidence-summary.md or evidence PDFs.");
        logsTextBox = NewReadOnlyMultilineBox("Launcher stdout/stderr logs will appear after a run.");

        AddTab(resultTabs, "Summary", phaseList);
        AddTab(resultTabs, "Diagnostics", diagnosticsList);
        AddTab(resultTabs, "Support Report", supportTextBox);
        AddTab(resultTabs, "Claim Boundary", claimBoundaryTextBox);
        AddTab(resultTabs, "Results", resultsTextBox);
        AddTab(resultTabs, "Oracle Compare", compareTextBox);
        AddTab(resultTabs, "Plots", plotsTextBox);
        AddTab(resultTabs, "Evidence", evidenceTextBox);
        AddTab(resultTabs, "Logs", logsTextBox);
        form.Controls.Add(resultTabs);

        Label footerLabel = new Label();
        footerLabel.Text = "Rusted EnergyPlus is not a drop-in replacement for EnergyPlus; SupportAssessment controls Rust execution, and oracle output is never shown as Rust success.";
        footerLabel.Location = new Point(18, 632);
        footerLabel.Size = new Size(810, 30);
        footerLabel.ForeColor = Color.DimGray;
        form.Controls.Add(footerLabel);

        timer = new Timer();
        timer.Interval = 500;
        timer.Tick += delegate {
            if (currentProcess != null && currentProcess.HasExited)
            {
                FinishRun();
            }
        };

        WireEvents();
    }

    private void WireEvents()
    {
        inputButton.Click += delegate {
            OpenFileDialog dialog = new OpenFileDialog();
            dialog.Filter = "EnergyPlus Inputs (*.idf;*.epJSON)|*.idf;*.epJSON|All files (*.*)|*.*";
            if (dialog.ShowDialog() == DialogResult.OK)
            {
                inputPath = dialog.FileName;
                SaveSettings();
                RefreshUi();
            }
        };
        weatherButton.Click += delegate {
            OpenFileDialog dialog = new OpenFileDialog();
            dialog.Filter = "Weather files (*.epw)|*.epw|All files (*.*)|*.*";
            if (dialog.ShowDialog() == DialogResult.OK)
            {
                weatherPath = dialog.FileName;
                SaveSettings();
                RefreshUi();
            }
        };
        outputButton.Click += delegate {
            FolderBrowserDialog dialog = new FolderBrowserDialog();
            dialog.SelectedPath = outputDir;
            if (dialog.ShowDialog() == DialogResult.OK)
            {
                outputDir = dialog.SelectedPath;
                SaveSettings();
                RefreshUi();
            }
        };
        oracleButton.Click += delegate {
            FolderBrowserDialog dialog = new FolderBrowserDialog();
            dialog.SelectedPath = oracleRoot;
            if (dialog.ShowDialog() == DialogResult.OK)
            {
                oracleRoot = dialog.SelectedPath;
                if (TestOracleRoot(oracleRoot))
                {
                    oracleBaseline = true;
                }
                SaveSettings();
                RefreshUi();
            }
        };
        exeButton.Click += delegate {
            OpenFileDialog dialog = new OpenFileDialog();
            dialog.Filter = "eplus-rs.exe|eplus-rs.exe|Executables (*.exe)|*.exe|All files (*.*)|*.*";
            if (dialog.ShowDialog() == DialogResult.OK)
            {
                eplusRsExe = dialog.FileName;
                SaveSettings();
                RefreshUi();
            }
        };
        compareButton.Click += delegate {
            compareOracle = !compareOracle;
            if (compareOracle)
            {
                oracleBaseline = true;
                outputFormat = "both";
            }
            SaveSettings();
            RefreshUi();
        };
        oracleBaselineButton.Click += delegate {
            if (compareOracle)
            {
                compareOracle = false;
                oracleBaseline = false;
            }
            else
            {
                oracleBaseline = !oracleBaseline;
            }
            SaveSettings();
            RefreshUi();
        };
        overwriteButton.Click += delegate {
            overwrite = !overwrite;
            SaveSettings();
            RefreshUi();
        };
        failOnWarningButton.Click += delegate {
            failOnWarning = !failOnWarning;
            SaveSettings();
            RefreshUi();
        };
        modeCombo.SelectedIndexChanged += delegate {
            if (!refreshingUi && modeCombo.SelectedItem != null)
            {
                mode = (string)modeCombo.SelectedItem;
                SaveSettings();
                RefreshUi();
            }
        };
        partialCombo.SelectedIndexChanged += delegate {
            if (!refreshingUi && partialCombo.SelectedItem != null)
            {
                partialPolicy = (string)partialCombo.SelectedItem;
                SaveSettings();
                RefreshUi();
            }
        };
        formatCombo.SelectedIndexChanged += delegate {
            if (!refreshingUi && formatCombo.SelectedItem != null)
            {
                outputFormat = (string)formatCombo.SelectedItem;
                SaveSettings();
                RefreshUi();
            }
        };
        traceCombo.SelectedIndexChanged += delegate {
            if (!refreshingUi && traceCombo.SelectedItem != null)
            {
                traceLevel = (string)traceCombo.SelectedItem;
                SaveSettings();
                RefreshUi();
            }
        };
        runButton.Click += delegate { StartRun(); };
        cancelButton.Click += delegate { CancelRun(); };
        openOutputButton.Click += delegate { OpenPath(outputDir); };
        openRunReportButton.Click += delegate { OpenPath(Path.Combine(outputDir, "reports", "run-report.md")); };
        openDiagnosticsButton.Click += delegate { OpenPath(Path.Combine(outputDir, "diagnostics.json")); };
        openSupportReportButton.Click += delegate { OpenPath(Path.Combine(outputDir, "support-report.md")); };
        openCompareButton.Click += delegate { OpenPath(Path.Combine(outputDir, "compare", "compare-report.md")); };
        openEvidenceButton.Click += delegate { OpenPath(FindEvidenceArtifactPath(outputDir)); };
        form.FormClosing += delegate(object sender, FormClosingEventArgs e) {
            if (currentProcess != null && !currentProcess.HasExited)
            {
                DialogResult answer = MessageBox.Show(
                    "A run is still active. Stop it and close?",
                    "Rusted EnergyPlus Launch",
                    MessageBoxButtons.YesNo,
                    MessageBoxIcon.Warning);
                if (answer != DialogResult.Yes)
                {
                    e.Cancel = true;
                    return;
                }
                currentProcess.Kill();
            }
        };
    }

    private void StartRun()
    {
        if (currentProcess != null)
        {
            return;
        }
        if (compareOracle && !TestOracleRoot(oracleRoot))
        {
            ShowError("Oracle compare needs an EnergyPlus 26.1.0 folder with energyplus.exe and ConvertInputFormat.exe.");
            return;
        }
        if (oracleBaseline && !TestOracleRoot(oracleRoot))
        {
            ShowError("Oracle baseline needs an EnergyPlus 26.1.0 folder with energyplus.exe and ConvertInputFormat.exe.");
            return;
        }
        bool weatherRequired = TestLauncherWeatherRequired(mode, oracleBaseline, compareOracle);
        if (weatherRequired && !File.Exists(weatherPath))
        {
            ShowError("The selected mode or oracle option needs a weather EPW file.");
            return;
        }
        if (compareOracle)
        {
            oracleBaseline = true;
            outputFormat = "both";
        }
        cancelRequested = false;
        SaveSettings();
        Directory.CreateDirectory(outputDir);

        List<string> arguments = NewLauncherRunArguments();
        ProcessStartInfo startInfo = new ProcessStartInfo();
        startInfo.FileName = eplusRsExe;
        startInfo.Arguments = JoinQuotedArguments(arguments);
        startInfo.UseShellExecute = false;
        startInfo.CreateNoWindow = true;
        startInfo.RedirectStandardOutput = true;
        startInfo.RedirectStandardError = true;

        Process process = new Process();
        process.StartInfo = startInfo;
        try
        {
            if (!process.Start())
            {
                ShowError("Failed to start eplus-rs.exe.");
                process.Dispose();
                return;
            }
        }
        catch (Exception error)
        {
            process.Dispose();
            ShowError("Failed to start eplus-rs.exe: " + error.Message);
            return;
        }

        currentProcess = process;
        stdoutTask = process.StandardOutput.ReadToEndAsync();
        stderrTask = process.StandardError.ReadToEndAsync();
        statusLabel.Text = "Running...";
        stateDetailLabel.Text = JoinQuotedArguments(arguments);
        stateDetailLabel.ForeColor = Color.DimGray;
        phaseList.Items.Clear();
        foreach (string stage in new string[] { "Input", "Convert", "RawModel", "TypedModel", "Graph", "Support", "Plan", "Runtime", "Export", "Oracle", "Compare" })
        {
            phaseList.Items.Add("queued: " + stage);
        }
        diagnosticsList.Items.Clear();
        diagnosticsList.Items.Add("Waiting for diagnostics.json.");
        RefreshUi();
        timer.Start();
    }

    private void CancelRun()
    {
        if (currentProcess == null)
        {
            return;
        }
        cancelRequested = true;
        statusLabel.Text = "Cancelling...";
        stateDetailLabel.Text = "Stopping eplus-rs run process.";
        stateDetailLabel.ForeColor = Color.DarkGoldenrod;
        try
        {
            if (!currentProcess.HasExited)
            {
                currentProcess.Kill();
            }
        }
        catch (Exception error)
        {
            ShowError("Failed to cancel eplus-rs.exe: " + error.Message);
        }
        RefreshUi();
    }

    private void FinishRun()
    {
        timer.Stop();
        int exitCode = currentProcess.ExitCode;
        string stdout = SafeTaskResult(stdoutTask);
        string stderr = SafeTaskResult(stderrTask);
        currentProcess.Dispose();
        currentProcess = null;

        if (Directory.Exists(outputDir))
        {
            string logsDir = Path.Combine(outputDir, "logs");
            Directory.CreateDirectory(logsDir);
            File.WriteAllText(Path.Combine(logsDir, "gui-stdout.log"), stdout, new UTF8Encoding(false));
            File.WriteAllText(Path.Combine(logsDir, "gui-stderr.log"), stderr, new UTF8Encoding(false));
        }

        Dictionary<string, object> summary = ReadJsonObject(Path.Combine(outputDir, "run-summary.json"));
        phaseList.Items.Clear();
        List<object> phases = GetPhases(summary);
        if (phases.Count == 0)
        {
            phaseList.Items.Add("No phase timing.");
        }
        else
        {
            int count = 0;
            foreach (object phase in phases)
            {
                if (count >= 12)
                {
                    break;
                }
                phaseList.Items.Add(FormatPhaseTimingLine(AsDictionary(phase)));
                count += 1;
            }
            object totalSeconds = GetNested(summary, new string[] { "timing", "total_wall_seconds" });
            if (totalSeconds != null)
            {
                phaseList.Items.Add(String.Format("total [{0:N3}s]", Convert.ToDouble(totalSeconds)));
            }
        }

        diagnosticsList.Items.Clear();
        List<object> diagnostics = ReadDiagnostics(Path.Combine(outputDir, "diagnostics.json"));
        if (diagnostics.Count == 0)
        {
            diagnosticsList.Items.Add("No diagnostics.");
        }
        else
        {
            int count = 0;
            foreach (object diagnostic in diagnostics)
            {
                if (count >= 8)
                {
                    break;
                }
                diagnosticsList.Items.Add(FormatDiagnosticLine(AsDictionary(diagnostic)));
                count += 1;
            }
        }

        supportTextBox.Text = ReadArtifactPreview(Path.Combine(outputDir, "support-report.md"), "Support report is not available for this run.");
        claimBoundaryTextBox.Text = "Claim boundary is not available until run-summary.json is written.";
        string selectedOutputsPath = Path.Combine(outputDir, "results", "selected-outputs.csv");
        string resultStorePath = Path.Combine(outputDir, "results", "result-store.json");
        string resultPreviewPath = File.Exists(selectedOutputsPath) ? selectedOutputsPath : resultStorePath;
        resultsTextBox.Text = ReadArtifactPreview(resultPreviewPath, "Rust result artifacts are not available for this run.");
        compareTextBox.Text = ReadArtifactPreview(Path.Combine(outputDir, "compare", "compare-report.md"), "Oracle compare report is not available for this run.");
        plotsTextBox.Text = ReadPlotArtifactPreview(outputDir);
        evidenceTextBox.Text = ReadEvidenceArtifactPreview(outputDir);
        logsTextBox.Text = "exit_code=" + exitCode + "\r\n\r\nstdout:\r\n" + stdout + "\r\n\r\nstderr:\r\n" + stderr;

        if (summary != null)
        {
            Presentation presentation = GetRunResultPresentation(summary);
            statusLabel.Text = presentation.Title;
            stateDetailLabel.Text = presentation.Detail;
            stateDetailLabel.ForeColor = Color.FromName(presentation.Color);
            claimBoundaryTextBox.Text = FormatClaimBoundaryText(presentation);
        }
        else if (cancelRequested)
        {
            statusLabel.Text = "Cancelled.";
            stateDetailLabel.Text = "Run process was cancelled before run-summary.json was written.";
            stateDetailLabel.ForeColor = Color.DarkGoldenrod;
        }
        else if (exitCode == 0)
        {
            statusLabel.Text = "Done.";
            stateDetailLabel.Text = "No run-summary.json was written.";
            stateDetailLabel.ForeColor = Color.DimGray;
        }
        else
        {
            statusLabel.Text = "Stopped with exit code " + exitCode + ".";
            stateDetailLabel.Text = "No run-summary.json was written.";
            stateDetailLabel.ForeColor = Color.Firebrick;
        }
        cancelRequested = false;
        RefreshUi();
    }

    private void RefreshUi()
    {
        refreshingUi = true;
        try
        {
            inputBox.Text = inputPath;
            weatherBox.Text = weatherPath;
            outputBox.Text = outputDir;
            oracleBox.Text = oracleRoot;
            exeBox.Text = eplusRsExe ?? "eplus-rs.exe not found";
            modeCombo.SelectedItem = mode;
            partialCombo.SelectedItem = partialPolicy;
            formatCombo.SelectedItem = outputFormat;
            traceCombo.SelectedItem = traceLevel;
            failOnWarningButton.Text = failOnWarning ? "Strict Warnings: ON" : "Strict Warnings: OFF";
            oracleBaselineButton.Text = (oracleBaseline || compareOracle) ? "Oracle Baseline: ON" : "Oracle Baseline: OFF";
            compareButton.Text = compareOracle ? "Oracle Compare: ON" : "Oracle Compare: OFF";
            overwriteButton.Text = overwrite ? "Overwrite: ON" : "Overwrite: OFF";

            bool isRunning = currentProcess != null;
            bool weatherRequired = TestLauncherWeatherRequired(mode, oracleBaseline, compareOracle);
            bool weatherReady = !weatherRequired || File.Exists(weatherPath);
            bool canRun = !isRunning &&
                !String.IsNullOrWhiteSpace(eplusRsExe) &&
                File.Exists(eplusRsExe) &&
                File.Exists(inputPath) &&
                weatherReady &&
                !String.IsNullOrWhiteSpace(outputDir);

            runButton.Enabled = canRun;
            cancelButton.Enabled = isRunning;
            inputButton.Enabled = !isRunning;
            weatherButton.Enabled = !isRunning;
            outputButton.Enabled = !isRunning;
            oracleButton.Enabled = !isRunning;
            exeButton.Enabled = !isRunning;
            modeCombo.Enabled = !isRunning;
            partialCombo.Enabled = !isRunning;
            formatCombo.Enabled = !isRunning;
            traceCombo.Enabled = !isRunning;
            failOnWarningButton.Enabled = !isRunning;
            oracleBaselineButton.Enabled = !isRunning;
            compareButton.Enabled = !isRunning;
            overwriteButton.Enabled = !isRunning;
            openOutputButton.Enabled = Directory.Exists(outputDir);
            openRunReportButton.Enabled = File.Exists(Path.Combine(outputDir, "reports", "run-report.md"));
            openDiagnosticsButton.Enabled = File.Exists(Path.Combine(outputDir, "diagnostics.json"));
            openSupportReportButton.Enabled = File.Exists(Path.Combine(outputDir, "support-report.md"));
            openCompareButton.Enabled = File.Exists(Path.Combine(outputDir, "compare", "compare-report.md"));
            openEvidenceButton.Enabled = File.Exists(FindEvidenceArtifactPath(outputDir));
        }
        finally
        {
            refreshingUi = false;
        }
    }

    private List<string> NewLauncherRunArguments()
    {
        List<string> arguments = new List<string>();
        arguments.Add("run");
        arguments.Add(inputPath);
        arguments.Add("-d");
        arguments.Add(outputDir);
        arguments.Add("--mode");
        arguments.Add(mode);
        arguments.Add("--partial");
        arguments.Add(partialPolicy);
        arguments.Add("--format");
        arguments.Add(outputFormat);
        arguments.Add("--trace-level");
        arguments.Add(traceLevel);
        if (!String.IsNullOrWhiteSpace(weatherPath))
        {
            arguments.Add("-w");
            arguments.Add(weatherPath);
        }
        if (failOnWarning)
        {
            arguments.Add("--fail-on-warning");
        }
        if (overwrite)
        {
            arguments.Add("--overwrite");
        }
        if (compareOracle)
        {
            arguments.Add("--compare-oracle");
        }
        else if (oracleBaseline)
        {
            arguments.Add("--oracle-baseline");
        }
        if (!String.IsNullOrWhiteSpace(oracleRoot))
        {
            arguments.Add("--oracle-root");
            arguments.Add(oracleRoot);
        }
        return arguments;
    }

    private string ResolveEplusRsExe()
    {
        List<string> candidates = new List<string>();
        candidates.Add(Path.Combine(appRoot, "bin", "eplus-rs.exe"));
        candidates.Add(Path.Combine(appRoot, "target", "debug", "eplus-rs.exe"));
        candidates.Add(Path.Combine(appRoot, "target", "release", "eplus-rs.exe"));
        if (!String.IsNullOrWhiteSpace(repoRoot))
        {
            candidates.Add(Path.Combine(repoRoot, "target", "debug", "eplus-rs.exe"));
            candidates.Add(Path.Combine(repoRoot, "target", "release", "eplus-rs.exe"));
        }
        string pathCandidate = ResolveCommandOnPath("eplus-rs.exe");
        if (!String.IsNullOrWhiteSpace(pathCandidate))
        {
            candidates.Add(pathCandidate);
        }
        foreach (string candidate in candidates)
        {
            if (File.Exists(candidate) && TestEplusRsRunCli(candidate))
            {
                return Path.GetFullPath(candidate);
            }
        }
        return null;
    }

    private bool TestEplusRsRunCli(string path)
    {
        ProcessStartInfo startInfo = new ProcessStartInfo();
        startInfo.FileName = path;
        startInfo.Arguments = "run";
        startInfo.UseShellExecute = false;
        startInfo.CreateNoWindow = true;
        startInfo.RedirectStandardOutput = true;
        startInfo.RedirectStandardError = true;
        Process process = new Process();
        process.StartInfo = startInfo;
        try
        {
            if (!process.Start())
            {
                return false;
            }
            if (!process.WaitForExit(5000))
            {
                process.Kill();
                return false;
            }
            string usageText = process.StandardOutput.ReadToEnd() + process.StandardError.ReadToEnd();
            return usageText.IndexOf("--mode compatibility|diagnostic", StringComparison.OrdinalIgnoreCase) >= 0 &&
                usageText.IndexOf("--partial deny|allow", StringComparison.OrdinalIgnoreCase) >= 0;
        }
        catch
        {
            return false;
        }
        finally
        {
            process.Dispose();
        }
    }

    private LauncherDefaults GetLauncherDefaultPaths()
    {
        string resolvedOracleRoot = ResolveOracleRoot();
        string idf = "";
        string weather = "";
        if (!String.IsNullOrWhiteSpace(resolvedOracleRoot))
        {
            string candidateIdf = Path.Combine(resolvedOracleRoot, "ExampleFiles", "1ZoneUncontrolled.idf");
            if (File.Exists(candidateIdf))
            {
                idf = Path.GetFullPath(candidateIdf);
            }
            string candidateWeather = Path.Combine(resolvedOracleRoot, "WeatherData", "USA_CO_Golden-NREL.724666_TMY3.epw");
            if (File.Exists(candidateWeather))
            {
                weather = Path.GetFullPath(candidateWeather);
            }
        }
        return new LauncherDefaults(resolvedOracleRoot, idf, weather);
    }

    private string ResolveOracleRoot()
    {
        List<string> candidates = new List<string>();
        candidates.Add(Environment.GetEnvironmentVariable("RUSTED_ENERGYPLUS_ORACLE_ROOT"));
        candidates.Add(Path.Combine(appRoot, "oracle", "energyplus", "26.1.0"));
        candidates.Add(Path.Combine(appRoot, ".runtime", "energyplus", "26.1.0"));
        if (!String.IsNullOrWhiteSpace(repoRoot))
        {
            candidates.Add(Path.Combine(repoRoot, ".runtime", "energyplus", "26.1.0"));
        }
        foreach (string candidate in candidates)
        {
            if (!String.IsNullOrWhiteSpace(candidate) && Directory.Exists(candidate))
            {
                return Path.GetFullPath(candidate);
            }
        }
        return "";
    }

    private bool TestOracleRoot(string path)
    {
        if (String.IsNullOrWhiteSpace(path))
        {
            return false;
        }
        return File.Exists(Path.Combine(path, "energyplus.exe")) &&
            File.Exists(Path.Combine(path, "ConvertInputFormat.exe"));
    }

    private static bool TestLauncherWeatherRequired(string mode, bool oracleBaseline, bool compareOracle)
    {
        return String.Equals(mode, "compatibility", StringComparison.OrdinalIgnoreCase) ||
            oracleBaseline ||
            compareOracle;
    }

    private void LoadSettings()
    {
        Dictionary<string, object> settings = ReadJsonObject(settingsPath);
        if (settings == null)
        {
            return;
        }
        inputPath = GetString(settings, "input_path", inputPath);
        weatherPath = GetString(settings, "weather_path", weatherPath);
        outputDir = GetString(settings, "output_dir", outputDir);
        oracleRoot = GetString(settings, "oracle_root", oracleRoot);
        string savedExe = GetString(settings, "eplus_rs_exe", "");
        if (!String.IsNullOrWhiteSpace(savedExe))
        {
            eplusRsExe = savedExe;
        }
        mode = GetString(settings, "mode", mode);
        partialPolicy = GetString(settings, "partial_policy", partialPolicy);
        outputFormat = GetString(settings, "output_format", outputFormat);
        traceLevel = GetString(settings, "trace_level", traceLevel);
        failOnWarning = GetBool(settings, "fail_on_warning", failOnWarning);
        oracleBaseline = GetBool(settings, "oracle_baseline", oracleBaseline);
        compareOracle = GetBool(settings, "compare_oracle", compareOracle);
        overwrite = GetBool(settings, "overwrite", overwrite);
    }

    private void SaveSettings()
    {
        string directory = Path.GetDirectoryName(settingsPath);
        if (!String.IsNullOrWhiteSpace(directory))
        {
            Directory.CreateDirectory(directory);
        }
        Dictionary<string, object> settings = new Dictionary<string, object>();
        settings["schema_version"] = 1;
        settings["input_path"] = inputPath;
        settings["weather_path"] = weatherPath;
        settings["output_dir"] = outputDir;
        settings["oracle_root"] = oracleRoot;
        settings["eplus_rs_exe"] = eplusRsExe ?? "";
        settings["mode"] = mode;
        settings["partial_policy"] = partialPolicy;
        settings["output_format"] = outputFormat;
        settings["trace_level"] = traceLevel;
        settings["fail_on_warning"] = failOnWarning;
        settings["oracle_baseline"] = oracleBaseline;
        settings["compare_oracle"] = compareOracle;
        settings["overwrite"] = overwrite;
        File.WriteAllText(settingsPath, json.Serialize(settings), new UTF8Encoding(false));
    }

    private Presentation GetRunResultPresentation(Dictionary<string, object> summary)
    {
        string status = GetString(summary, "status", "unknown");
        string exitCode = GetObjectString(GetValue(summary, "exit_code"), "unknown");
        string oracleStatus = GetString(summary, "oracle_status", "not-run");
        string compareStatus = GetString(summary, "compare_status", "not-run");
        string runMode = GetNestedString(summary, new string[] { "config", "mode" }, "unknown");
        string supportReportPath = GetNestedString(summary, new string[] { "artifacts", "support_report_md" }, "support-report.md");
        string selectedOutputsPath = GetNestedString(summary, new string[] { "artifacts", "selected_outputs_csv" }, "");
        string resultStorePath = GetNestedString(summary, new string[] { "artifacts", "result_store_json" }, "");
        string compareReportPath = GetNestedString(summary, new string[] { "artifacts", "compare_report_md" }, "");
        string runState = GetNestedString(summary, new string[] { "support", "run_result_state" }, "unknown");
        string supportStatus = GetNestedString(summary, new string[] { "support", "status" }, GetString(summary, "support_status", "unknown"));
        string runtimeClass = GetNestedString(summary, new string[] { "support", "runtime_class" }, "unknown");
        string capabilityText = JoinStringArray(GetNested(summary, new string[] { "support", "matched_capability_ids" }));
        if (String.IsNullOrWhiteSpace(capabilityText))
        {
            capabilityText = "none";
        }

        string title = "Run status unknown";
        string color = "DimGray";
        string stateMessage = "Run status could not be classified.";
        if (runState == "run_blocked")
        {
            title = "Simulation was not run.";
            color = "Firebrick";
            stateMessage = "Simulation was not run. Top unsupported reasons are in support-report.md.";
        }
        else if (runState == "partial_supported_run")
        {
            title = "Ad-hoc partial run, not conformance evidence.";
            color = "DarkGoldenrod";
            stateMessage = "Ad-hoc partial run, not conformance evidence. Ignored or inactive objects are listed in support-report.md.";
        }
        else if (runState == "supported_compatibility_run")
        {
            title = "Supported compatibility run";
            color = "ForestGreen";
            stateMessage = "Matched capabilities selected the supported compatibility runtime; arbitrary runs still keep conformance_claim=false.";
        }
        if (runMode == "diagnostic" && runState == "partial_supported_run")
        {
            stateMessage += " Diagnostic-only execution is explicit.";
        }
        else if (runMode == "fast" || runMode == "experimental")
        {
            stateMessage += " Fast and experimental modes are never release conformance evidence.";
        }
        string resultPath = !String.IsNullOrWhiteSpace(selectedOutputsPath) ? selectedOutputsPath : resultStorePath;
        string detail = String.Join("; ", new string[] {
            stateMessage,
            "status=" + status,
            "exit_code=" + exitCode,
            "mode=" + runMode,
            "support=" + supportStatus,
            "runtime=" + runtimeClass,
            "oracle=" + oracleStatus,
            "compare=" + compareStatus,
            "matched_capabilities=" + capabilityText,
            "claim_boundary=ad-hoc arbitrary run",
            "conformance_claim=false",
            "support_report=" + supportReportPath,
            "results=" + resultPath,
            "compare_report=" + compareReportPath
        });
        return new Presentation(runState, title, color, detail);
    }

    private static string FormatClaimBoundaryText(Presentation presentation)
    {
        return "Claim Boundary\r\n\r\n" +
            presentation.Detail +
            "\r\n\r\nFast and experimental modes are never release conformance evidence.";
    }

    private List<object> GetPhases(Dictionary<string, object> summary)
    {
        List<object> phases = new List<object>();
        object value = GetNested(summary, new string[] { "timing", "phases" });
        object[] array = value as object[];
        if (array != null)
        {
            phases.AddRange(array);
        }
        return phases;
    }

    private List<object> ReadDiagnostics(string path)
    {
        List<object> diagnostics = new List<object>();
        Dictionary<string, object> payload = ReadJsonObject(path);
        object value = GetValue(payload, "diagnostics");
        object[] array = value as object[];
        if (array != null)
        {
            diagnostics.AddRange(array);
        }
        return diagnostics;
    }

    private string FormatDiagnosticLine(Dictionary<string, object> diagnostic)
    {
        string severity = GetString(diagnostic, "severity", "unknown");
        string code = GetString(diagnostic, "code", "Diagnostic");
        string stage = GetString(diagnostic, "stage", "unknown");
        string message = GetString(diagnostic, "message", "");
        return severity + " [" + code + "] " + stage + ": " + message;
    }

    private string FormatPhaseTimingLine(Dictionary<string, object> phase)
    {
        string name = GetString(phase, "name", "unknown");
        string engine = GetString(phase, "engine", "unknown");
        string scope = GetString(phase, "scope", "");
        object seconds = GetValue(phase, "wall_seconds");
        string secondsText = seconds == null ? "n/a" : String.Format("{0:N3}s", Convert.ToDouble(seconds));
        if (String.IsNullOrWhiteSpace(scope))
        {
            return name + " [" + engine + "] " + secondsText;
        }
        return name + " [" + engine + "] " + secondsText + " - " + scope;
    }

    private string ReadArtifactPreview(string path, string missingText)
    {
        if (String.IsNullOrWhiteSpace(path) || !File.Exists(path))
        {
            return missingText;
        }
        try
        {
            string text = File.ReadAllText(path, Encoding.UTF8);
            if (text.Length > 12000)
            {
                return text.Substring(0, 12000) + "\r\n\r\n...";
            }
            return text;
        }
        catch (Exception error)
        {
            return "Unable to read artifact: " + error.Message;
        }
    }

    private string ReadPlotArtifactPreview(string output)
    {
        List<string> roots = new List<string>();
        roots.Add(Path.Combine(output, "reports", "plots"));
        roots.Add(Path.Combine(output, "plots"));
        roots.Add(Path.Combine(output, "compare", "plots"));
        StringBuilder builder = new StringBuilder();
        foreach (string root in roots)
        {
            if (!Directory.Exists(root))
            {
                continue;
            }
            foreach (string file in Directory.GetFiles(root, "*.png"))
            {
                FileInfo info = new FileInfo(file);
                builder.AppendLine(info.FullName + " (" + info.Length + " bytes)");
            }
        }
        if (builder.Length == 0)
        {
            return "Plot artifacts will appear after a run writes reports\\plots, plots, or compare\\plots.";
        }
        return builder.ToString();
    }

    private string FindEvidenceArtifactPath(string output)
    {
        if (String.IsNullOrWhiteSpace(output))
        {
            return "";
        }
        string summary = Path.Combine(output, "reports", "evidence-summary.md");
        if (File.Exists(summary))
        {
            return summary;
        }
        string pdf = Path.Combine(output, "reports", "numeric-conformance-evidence.pdf");
        if (File.Exists(pdf))
        {
            return pdf;
        }
        return "";
    }

    private string ReadEvidenceArtifactPreview(string output)
    {
        string path = FindEvidenceArtifactPath(output);
        StringBuilder builder = new StringBuilder();
        builder.AppendLine("Evidence artifacts");
        if (String.IsNullOrWhiteSpace(path))
        {
            builder.AppendLine("No evidence summary or PDF is available for this run.");
            return builder.ToString();
        }
        builder.AppendLine(path);
        string reports = Path.Combine(output, "reports");
        if (Directory.Exists(reports))
        {
            foreach (string file in Directory.GetFiles(reports, "*.pdf"))
            {
                builder.AppendLine(file);
            }
        }
        if (path.EndsWith(".md", StringComparison.OrdinalIgnoreCase))
        {
            builder.AppendLine();
            builder.AppendLine(ReadArtifactPreview(path, ""));
        }
        return builder.ToString();
    }

    private void OpenPath(string path)
    {
        if (String.IsNullOrWhiteSpace(path) || (!File.Exists(path) && !Directory.Exists(path)))
        {
            ShowError("File or folder is not available yet.");
            return;
        }
        ProcessStartInfo startInfo = new ProcessStartInfo();
        startInfo.FileName = path;
        startInfo.UseShellExecute = true;
        Process.Start(startInfo);
    }

    private Dictionary<string, object> ReadJsonObject(string path)
    {
        if (String.IsNullOrWhiteSpace(path) || !File.Exists(path))
        {
            return null;
        }
        try
        {
            object value = json.DeserializeObject(File.ReadAllText(path, Encoding.UTF8));
            return value as Dictionary<string, object>;
        }
        catch
        {
            return null;
        }
    }

    private static Dictionary<string, object> AsDictionary(object value)
    {
        Dictionary<string, object> dictionary = value as Dictionary<string, object>;
        if (dictionary == null)
        {
            return new Dictionary<string, object>();
        }
        return dictionary;
    }

    private static object GetValue(Dictionary<string, object> dictionary, string name)
    {
        if (dictionary == null || !dictionary.ContainsKey(name))
        {
            return null;
        }
        return dictionary[name];
    }

    private static object GetNested(Dictionary<string, object> dictionary, string[] names)
    {
        object current = dictionary;
        foreach (string name in names)
        {
            Dictionary<string, object> currentDictionary = current as Dictionary<string, object>;
            if (currentDictionary == null || !currentDictionary.ContainsKey(name))
            {
                return null;
            }
            current = currentDictionary[name];
        }
        return current;
    }

    private static string GetNestedString(Dictionary<string, object> dictionary, string[] names, string fallback)
    {
        return GetObjectString(GetNested(dictionary, names), fallback);
    }

    private static string GetString(Dictionary<string, object> dictionary, string name, string fallback)
    {
        return GetObjectString(GetValue(dictionary, name), fallback);
    }

    private static bool GetBool(Dictionary<string, object> dictionary, string name, bool fallback)
    {
        object value = GetValue(dictionary, name);
        if (value == null)
        {
            return fallback;
        }
        if (value is bool)
        {
            return (bool)value;
        }
        bool parsed;
        if (Boolean.TryParse(Convert.ToString(value), out parsed))
        {
            return parsed;
        }
        return fallback;
    }

    private static string GetObjectString(object value, string fallback)
    {
        if (value == null)
        {
            return fallback;
        }
        string text = Convert.ToString(value);
        if (String.IsNullOrWhiteSpace(text))
        {
            return fallback;
        }
        return text;
    }

    private static string JoinStringArray(object value)
    {
        object[] array = value as object[];
        if (array == null || array.Length == 0)
        {
            return "";
        }
        List<string> parts = new List<string>();
        foreach (object item in array)
        {
            parts.Add(Convert.ToString(item));
        }
        return String.Join(", ", parts.ToArray());
    }

    private static string SafeTaskResult(Task<string> task)
    {
        if (task == null)
        {
            return "";
        }
        try
        {
            return task.Result ?? "";
        }
        catch (Exception error)
        {
            return "Unable to read process output: " + error.Message;
        }
    }

    private static string JoinQuotedArguments(List<string> arguments)
    {
        List<string> quoted = new List<string>();
        foreach (string argument in arguments)
        {
            quoted.Add(QuoteProcessArgument(argument));
        }
        return String.Join(" ", quoted.ToArray());
    }

    private static string QuoteProcessArgument(string value)
    {
        if (String.IsNullOrEmpty(value))
        {
            return "\"\"";
        }
        bool needsQuotes = false;
        foreach (char character in value)
        {
            if (Char.IsWhiteSpace(character) || character == '"')
            {
                needsQuotes = true;
                break;
            }
        }
        if (!needsQuotes)
        {
            return value;
        }
        StringBuilder builder = new StringBuilder();
        builder.Append('"');
        int backslashes = 0;
        foreach (char character in value)
        {
            if (character == '\\')
            {
                backslashes += 1;
                continue;
            }
            if (character == '"')
            {
                builder.Append('\\', (backslashes * 2) + 1);
                builder.Append('"');
                backslashes = 0;
                continue;
            }
            if (backslashes > 0)
            {
                builder.Append('\\', backslashes);
                backslashes = 0;
            }
            builder.Append(character);
        }
        if (backslashes > 0)
        {
            builder.Append('\\', backslashes * 2);
        }
        builder.Append('"');
        return builder.ToString();
    }

    private static string ResolveCommandOnPath(string fileName)
    {
        string path = Environment.GetEnvironmentVariable("PATH");
        if (String.IsNullOrWhiteSpace(path))
        {
            return "";
        }
        foreach (string directory in path.Split(Path.PathSeparator))
        {
            if (String.IsNullOrWhiteSpace(directory))
            {
                continue;
            }
            string candidate = Path.Combine(directory.Trim(), fileName);
            if (File.Exists(candidate))
            {
                return candidate;
            }
        }
        return "";
    }

    private static string ResolveRepoRoot(string root)
    {
        DirectoryInfo directory = new DirectoryInfo(root);
        while (directory != null)
        {
            if (File.Exists(Path.Combine(directory.FullName, "Cargo.toml")) &&
                Directory.Exists(Path.Combine(directory.FullName, "scripts")))
            {
                return directory.FullName;
            }
            directory = directory.Parent;
        }
        return "";
    }

    private static string NullToEmpty(string value)
    {
        return value ?? "";
    }

    private static Button NewButton(string text, int x, int y, int width, int height)
    {
        Button button = new Button();
        button.Text = text;
        button.Location = new Point(x, y);
        button.Size = new Size(width, height);
        button.FlatStyle = FlatStyle.System;
        return button;
    }

    private static TextBox NewPathBox(int y)
    {
        TextBox box = new TextBox();
        box.Location = new Point(178, y);
        box.Size = new Size(650, 24);
        box.ReadOnly = true;
        return box;
    }

    private static Label NewLabel(string text, int x, int y, int width, int height)
    {
        Label label = new Label();
        label.Text = text;
        label.Location = new Point(x, y);
        label.Size = new Size(width, height);
        label.TextAlign = ContentAlignment.MiddleLeft;
        return label;
    }

    private static ComboBox NewComboBox(string[] items, string selected, int x, int y, int width, int height)
    {
        ComboBox combo = new ComboBox();
        combo.DropDownStyle = ComboBoxStyle.DropDownList;
        combo.Location = new Point(x, y);
        combo.Size = new Size(width, height);
        combo.Items.AddRange(items);
        combo.SelectedItem = selected;
        return combo;
    }

    private static ListBox NewListBox(string text)
    {
        ListBox list = new ListBox();
        list.Dock = DockStyle.Fill;
        list.HorizontalScrollbar = true;
        list.Items.Add(text);
        return list;
    }

    private static TextBox NewReadOnlyMultilineBox(string text)
    {
        TextBox box = new TextBox();
        box.Dock = DockStyle.Fill;
        box.Multiline = true;
        box.ReadOnly = true;
        box.ScrollBars = ScrollBars.Both;
        box.WordWrap = false;
        box.Text = text;
        return box;
    }

    private static void AddTab(TabControl tabs, string title, Control control)
    {
        TabPage page = new TabPage();
        page.Text = title;
        page.Controls.Add(control);
        tabs.TabPages.Add(page);
    }

    private static void ShowError(string message)
    {
        MessageBox.Show(
            message,
            "Rusted EnergyPlus Launch",
            MessageBoxButtons.OK,
            MessageBoxIcon.Error);
    }
}

internal sealed class LauncherDefaults
{
    public readonly string OracleRoot;
    public readonly string Idf;
    public readonly string Weather;

    public LauncherDefaults(string oracleRoot, string idf, string weather)
    {
        OracleRoot = oracleRoot;
        Idf = idf;
        Weather = weather;
    }
}

internal sealed class Presentation
{
    public readonly string StateId;
    public readonly string Title;
    public readonly string Color;
    public readonly string Detail;

    public Presentation(string stateId, string title, string color, string detail)
    {
        StateId = stateId;
        Title = title;
        Color = color;
        Detail = detail;
    }
}
