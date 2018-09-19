import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.checkpoint.CheckpointFactory as CheckpointFactory
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as MobileBuiltInKeywords
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testcase.TestCaseFactory as TestCaseFactory
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testdata.TestDataFactory as TestDataFactory
import com.kms.katalon.core.testobject.ObjectRepository as ObjectRepository
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WSBuiltInKeywords
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUiBuiltInKeywords
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import internal.GlobalVariable as GlobalVariable
import org.openqa.selenium.Keys as Keys

//File pathToBinary = new File("C:\\Program Files\\Mozilla Firefox15\\Firefox.exe");
//FirefoxBinary ffBinary = new FirefoxBinary(pathToBinary);
//FirefoxProfile firefoxProfile = new FirefoxProfile();
//FirefoxDriver _driver = new FirefoxDriver(ffBinary,firefoxProfile);
WebUI.openBrowser('')

WebUI.navigateToUrl('http://mauw2k8sdworx:81/')

WebUI.setText(findTestObject('Temp/Page_ (17)/input_Username'), 'dev@sdworx.com')

WebUI.setText(findTestObject('Temp/Page_ (17)/input_Password'), '1234')

WebUI.click(findTestObject('Temp/Page_ (17)/input_Submit'))

WebUI.maximizeWindow()

WebUI.click(findTestObject('Temp/Page_Implementation Toolkit (19)/h4_YOUR SETUP'))

WebUI.click(findTestObject('Temp/Page_Implementation Toolkit (19)/button_Your Setup'))

WebUI.click(findTestObject('Temp/Page_Implementation Toolkit (19)/a_Your Payroll'))

WebUI.click(findTestObject('Temp/Page_Implementation Toolkit (19)/a_Pay Groups'))

WebUI.selectOptionByValue(findTestObject('Temp/select_111111 - test981078 - D'), '30e66e53-4f3c-4a03-9e1b-191b06a117a3', true)

for (def row = 1; row <= findTestData('Paygroup').getRowNumbers(); row++) {
    WebUI.click(findTestObject('Temp/19032018/a_Add Paygroup'))

    WebUI.setText(findTestObject('Temp/New Folder/19032018/input_PaygroupValue0'), findTestData('Paygroup').getValue(1, row))

    WebUI.setText(findTestObject('Temp/New Folder/19032018/input_PaygroupDescription0'), findTestData('Paygroup').getValue(2, 
            row))

    WebUI.selectOptionByValue(findTestObject('Temp/select_Please select5448-TaxAu'), 'ad6e449f-3a63-480b-9c8b-2a2bbfef344e', 
        true)

    WebUI.click(findTestObject('Temp/Page_Implementation Toolkit (19)/button_Save'))
}

