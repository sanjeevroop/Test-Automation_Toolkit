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

//WebUI.openBrowser('https://implementationnp.sdworx.co.uk/')

//WebUI.navigateToUrl('https://implementationnp.sdworx.co.uk/')

//WebUI.setText(findTestObject('Fields Validation/Department Details/Invalid Entry - Department Value/input_loginfmt'), 'Sanjeev@sdworx.com')

//WebUI.click(findTestObject('Fields Validation/Department Details/Invalid Entry - Department Value/input_idSIButton9'))

//WebUI.delay(3)

//WebUI.mouseOver(findTestObject('Fields Validation/Department Details/Invalid Entry - Department Value/button_Your Setup'))

//WebUI.delay(4)

//WebUI.click(findTestObject('Fields Validation/Department Details/Invalid Entry - Department Value/button_Your Setup'))

//WebUI.delay(5)

//WebUI.mouseOver(findTestObject('Fields Validation/Department Details/Invalid Entry - Department Value/a_Your Payroll'))

//WebUI.delay(5)

//WebUI.click(findTestObject('Fields Validation/Department Details/Invalid Entry - Department Value/a_Departments'))

WebUI.delay(2)

WebUI.click(findTestObject('Fields Validation/Department Details/Invalid Entry - Department Value/a_Add Department'))

WebUI.delay(2)

WebUI.setText(findTestObject('Fields Validation/Department Details/Invalid Entry - Department Value/input_DepartmentNumber0'), '123r')

WebUI.delay(1)

WebUI.setText(findTestObject('Fields Validation/Department Details/Invalid Entry - Department Value/input_DepartmentDescription0'), 'Auto1')

WebUI.delay(1)

WebUI.setText(findTestObject('Fields Validation/Department Details/Invalid Entry - Department Value/input_ReferenceCode0'), '239')

WebUI.delay(1)

WebUI.click(findTestObject('Fields Validation/Department Details/Invalid Entry - Department Value/button_Save'))


WebUI.verifyElementText(findTestObject('Fields Validation/Department Details/Invalid Entry - Department Value/span_Field should be 4 numeric'),
	'Field should be 4 numeric characters')

//WebUI.delay(2)

//WebUI.takeScreenshot('Screenshot/Department Details Screen/DepartmentDetails6.png')

//WebUI.setText(findTestObject('Fields Validation/Invalid Entry - Department Details/input_DepartmentNumber0'), '1233')

//WebUI.setText(findTestObject('Fields Validation/Invalid Entry - Department Details/input_DepartmentDescription0'), 'Test123456tyrtyui')

//WebUI.click(findTestObject('Fields Validation/Invalid Entry - Department Details/button_Save'))


//WebUI.click(findTestObject('Fields Validation/Invalid Entry - Department Details/span_Field should be 16 charac'))

//WebUI.click(findTestObject('Fields Validation/Invalid Entry - Department Details/span_Field should be 16 charac'))

//WebUI.rightClick(findTestObject('Fields Validation/Invalid Entry - Department Details/span_Field should be 16 charac'))

//WebUI.setText(findTestObject('Fields Validation/Invalid Entry - Department Details/input_DepartmentDescription0'), 'Test123456tyrtyu')

//WebUI.setText(findTestObject('Fields Validation/Invalid Entry - Department Details/input_DepartmentNumber0'), '123r')

//WebUI.click(findTestObject('Fields Validation/Invalid Entry - Department Details/button_Save'))

//WebUI.click(findTestObject('null'))

//WebUI.rightClick(findTestObject('null'))


//WebUI.setText(findTestObject('Fields Validation/Invalid Entry - Department Details/input_DepartmentNumber0'), '1232')

////WebUI.setText(findTestObject('Fields Validation/Invalid Entry - Department Details/input_DepartmentDescription0'), 'Test123456tyrty%')

//WebUI.click(findTestObject('Fields Validation/Invalid Entry - Department Details/button_Save'))


