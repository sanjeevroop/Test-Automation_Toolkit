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

//WebUI.delay(5)

//WebUI.mouseOver(findTestObject('Fields Validation/Department Details/Invalid Entry - Department Value/button_Your Setup'))

//WebUI.delay(5)

//WebUI.click(findTestObject('Fields Validation/Department Details/Invalid Entry - Department Value/button_Your Setup'))

//WebUI.delay(3)

//WebUI.mouseOver(findTestObject('Fields Validation/Department Details/Invalid Entry - Department Value/a_Your Payroll'))

//WebUI.delay(3)

//WebUI.click(findTestObject('Fields Validation/Department Details/Invalid Entry - Department Value/a_Departments'))

WebUI.delay(2)

WebUI.click(findTestObject('Object Repository/Fields Validation/Department Details/Invalid Entry - Department Description/a_Add Department (1)'))

WebUI.delay(2)

//WebUI.setText(findTestObject('Fields Validation/Department Details/Invalid Entry - Department Value/input_DepartmentNumber0'), '123')

//WebUI.delay(2)

//WebUI.setText(findTestObject('Fields Validation/Invalid Entry - Department Value/input_DepartmentDescription0'), 'Test')

//WebUI.delay(2)

//WebUI.setText(findTestObject('Fields Validation/Department Details/Invalid Entry - Department Value/input_ReferenceCode0'), '23')

//WebUI.delay(2)

//WebUI.click(findTestObject('Fields Validation/Department Details/Invalid Entry - Department Value/button_Save'))


//WebUI.verifyElementText(findTestObject('Fields Validation/Department Details/Invalid Entry - Department Value/span_Field should be 4 numeric'),
	//'Field should be 16 characters alphanumeric description')

WebUI.setText(findTestObject('Fields Validation/Department Details/Invalid Entry - Department Description/input_DepartmentNumber0'), '1233')

WebUI.delay(2)

WebUI.setText(findTestObject('Fields Validation/Department Details/Invalid Entry - Department Description/input_DepartmentDescription0'), 'Test123456tyrty$')

WebUI.delay(2)

WebUI.setText(findTestObject('Fields Validation/Department Details/Invalid Entry - Department Description/input_ReferenceCode0'), '234')

WebUI.delay(2)

WebUI.click(findTestObject('Object Repository/Fields Validation/Department Details/Invalid Entry - Department Description/button_Save'))

WebUI.verifyElementText(findTestObject('Fields Validation/Department Details/Invalid Entry - Department Description/span_Field should be 16 charac'),
'Field should be 16 characters alphanumeric description')


//WebUI.delay(2)

//WebUI.takeScreenshot('Screenshot/Department Details Screen/DepartmentDetails8.png')

//WebUI.setText(findTestObject('Fields Validation/Invalid Entry - Department Details/input_DepartmentDescription0'), 'Test123456tyrtyu')

//WebUI.setText(findTestObject('Fields Validation/Invalid Entry - Department Details/input_DepartmentNumber0'), '123r')

//WebUI.setText(findTestObject('Fields Validation/Department Details/Invalid Entry - Department Value/input_ReferenceCode0'), '23')

//WebUI.click(findTestObject('Fields Validation/Invalid Entry - Department Details/button_Save'))




//WebUI.setText(findTestObject('Fields Validation/Invalid Entry - Department Details/input_DepartmentNumber0'), '1232')

////WebUI.setText(findTestObject('Fields Validation/Invalid Entry - Department Details/input_DepartmentDescription0'), 'Test123456tyrty%')

//WebUI.click(findTestObject('Fields Validation/Invalid Entry - Department Details/button_Save'))


