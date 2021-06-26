using System;
using Device.Net;
using Microsoft.Extensions.Logging;
using Hid.Net.Windows;
using System.Threading.Tasks;
using Usb.Net.Windows;
using System.Linq;
using System.Threading;
using System.Collections.Generic;
using Device.Net.Windows;

namespace PicoRGB.Configure
{
    public class Report : IRequest
    {
        public byte[] ToArray()
        {
            var buffer = new byte[64];
            for (int i = 0; i < buffer.Length; i++)
            {
                buffer[i] = (byte)i;
            }
            return new byte[64];
        }
    }

    class Program
    {
        public static void OnDeviceNotify(ConnectedDeviceDefinition connectedDevices)
        {
            logger.LogCritical("OnDeviceNotify");
        }

        public static void OnDevicesNotify(IReadOnlyCollection<ConnectedDeviceDefinition> connectedDevices)
        {
            logger.LogCritical("OnDevicesNotify");
            foreach (var device in connectedDevices)
            {
                Console.WriteLine(device.DeviceId);
                deviceManager.SelectDevice(device);
            }
        }

        public static void OnNotifyDeviceException(ConnectedDeviceDefinition connectedDevice, Exception exception)
        {
            logger.LogCritical("OnNotifyDeviceException");
        }

        private static Lazy<IDeviceFactory> hidFactory = new Lazy<IDeviceFactory>(() => {
            var filter = new FilterDeviceDefinition(vendorId: 0x1209, productId: 0x4004, label: "PicoRGB");
            var hidFactory = filter.CreateWindowsHidDeviceFactory(loggerFactory);

            //Register the factory for creating Usb devices.
            var usbFactory = filter.CreateWindowsUsbDeviceFactory(loggerFactory);

            var factories = hidFactory.Aggregate(usbFactory);

            return factories;
            }
        );

        public static async Task<IReadOnlyList<ConnectedDeviceDefinition>> OnGetConnectedDevicesAsync()
        {
            logger.LogCritical("OnGetConnectedDevicesAsync");
            // // Console.WriteLine("Scanning devices...");

            //Register the factory for creating Usb devices.
            // var usbFactory = new FilterDeviceDefinition(vendorId: 0x1209, productId: 0x4004)
            //     .CreateWindowsUsbDeviceFactory(loggerFactory);

            // var factories = hidFactory.Aggregate(usbFactory);

            return (await hidFactory.Value.GetConnectedDeviceDefinitionsAsync()).ToList();
            // return Enumerable.Empty<ConnectedDeviceDefinition>().ToList();
        }

        public static async Task OnInitializeDeviceAction(IDevice device)
        {
            logger.LogCritical("OnInitializeDeviceAction");
            await device.InitializeAsync();
        }

        public static async Task<IDevice> OnGetDeviceAsync(ConnectedDeviceDefinition deviceId, CancellationToken cancellationToken = default)
        {

            //Register the factory for creating Usb devices.
            // var usbFactory = new FilterDeviceDefinition(vendorId: 0x1209, productId: 0x4004)
            //     .CreateWindowsUsbDeviceFactory(loggerFactory);

            // var factories = hidFactory.Aggregate(usbFactory);

            return await hidFactory.Value.GetDeviceAsync(deviceId, cancellationToken);
            // return (await factories.GetConnectedDeviceDefinitionsAsync()).ToList();
            // logger.LogCritical("OnGetDeviceAsync");
            // return null;
        }

        private static ILoggerFactory loggerFactory;
        private static DeviceManager deviceManager;
        private static ILogger<Program> logger;

        static async Task Main(string[] args)
        {
            loggerFactory = LoggerFactory.Create((builder) =>
            {
                _ = builder.AddConsole().SetMinimumLevel(LogLevel.Trace);
            });

            var tokenSource = new CancellationTokenSource(TimeSpan.FromSeconds(1));
            var deviceDefinition = (await hidFactory.Value.GetConnectedDeviceDefinitionsAsync(tokenSource.Token)).Single();
            var device = await hidFactory.Value.GetDeviceAsync(deviceDefinition, tokenSource.Token);
            await device.InitializeAsync(tokenSource.Token);

            //deviceDefinition.
            //logger = loggerFactory.CreateLogger<Program>();
            //deviceManager = new DeviceManager(OnDeviceNotify, OnDevicesNotify, OnNotifyDeviceException, OnInitializeDeviceAction, OnGetConnectedDevicesAsync, OnGetDeviceAsync, 1000, loggerFactory);
            //deviceManager.Start();

            //while (deviceManager.SelectedDevice == null)
            //{
            //    // Console.WriteLine("Device not selected...");
            //    await Task.Delay(1000);
            //}

            Console.WriteLine("TRYING TO USE THE FUCKING DEVICE");
            var foo = new byte[65];
            foo[0] = 2;
            foo[1] = 24;
            foo[1] = 53;
            var result = await device.WriteAndReadAsync(foo, tokenSource.Token);
            Console.WriteLine(string.Join(',', result.Data));
            Console.WriteLine("TRYING TO USE THE FUCKING DEVICE");

            //var report = await deviceManager.WriteAndReadAsync(new Report(), (buffer) => buffer);
            //foreach (var b in report)
            //{
            //    Console.Write(b);
            //}
            // deviceManager.WriteAndReadAsync

            //while (true)
            //{
            //    // Console.WriteLine("Scanning devices...");
            //    // var loggerFactory = LoggerFactory.Create((builder) =>
            //    // {
            //    //     _ = builder.AddDebug().SetMinimumLevel(LogLevel.Trace);
            //    // });

            //    // var hidFactory = new FilterDeviceDefinition(vendorId: 0x1209)
            //    //     .CreateWindowsHidDeviceFactory(loggerFactory);

            //    // //Register the factory for creating Usb devices.
            //    // var usbFactory = new FilterDeviceDefinition(vendorId: 0x1209)
            //    //     .CreateWindowsUsbDeviceFactory(loggerFactory);

            //    // var factories = hidFactory.Aggregate(usbFactory);

            //    // var deviceDefinition = await factories.GetConnectedDeviceDefinitionsAsync();

            //    // Console.WriteLine("Found device: {0}", deviceDefinition.First().ProductName);

            //    // var device = await hidFactory.GetDeviceAsync(deviceDefinition.First());
            //    // Console.WriteLine("Initializing...");
            //    // await device.InitializeAsync();
            //    // var buffer = System.Text.Encoding.ASCII.GetBytes(new string('x', 64));
            //    // buffer[0] = 0;
            //    // // Console.WriteLine((await device.ReadAsync()).BytesTransferred);
            //    // var tokenSource = new CancellationTokenSource();
            //    // tokenSource.CancelAfter(1000);
            //    // // device.
            //    // var readBuffer = await device.WriteAndReadAsync(buffer, tokenSource.Token);
            //    // Console.WriteLine(readBuffer.BytesTransferred);
            //    await Task.Delay(1000);
            //}
        }
    }
}
