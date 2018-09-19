import org.openqa.selenium.Keys as Keys
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import internal.GlobalVariable as GlobalVariable

WebUI.openBrowser('https://implementationnp.sdworx.co.uk/')

WebUI.navigateToUrl('https://implementationnp.sdworx.co.uk/')

//WebUI.setText(findTestObject('Edit Records/Archives/Edit - Department Details Cancel/input_loginfmt'), '')

//WebUI.click(findTestObject('Edit Records/Archives/Edit - Department Details Cancel/div_Sign in'))

WebUI.setText(findTestObject('Edit Records/Archives/Edit - Department Details Cancel/input_loginfmt'), 'Sanjeev@sdworx.com')

WebUI.click(findTestObject('Edit Records/Archives/Edit - Department Details Cancel/input_idSIButton9'))

//WebUI.delay(5)

//WebUI.click(findTestObject('Edit Records/Archives/Edit - Department Details Cancel/button_Your Setup'))

WebUI.delay(7)

WebUI.mouseOver(findTestObject('Edit Records/Archives/Edit - Department Details Cancel/button_Your Setup'))

WebUI.delay(2)

WebUI.mouseOver(findTestObject('Edit Records/Archives/Edit - Department Details Cancel/a_Your Payroll'))

WebUI.delay(2)

WebUI.click(findTestObject('Edit Records/Archives/Edit - Department Details Cancel/a_Departments'))

WebUI.delay(2)

//WebUI.click(findTestObject('Edit Records/Archives/Edit - Department Details Cancel/a_Edit'))

WebUI.click(findTestObject('Edit Records/Archives/Edit - Department Details Cancel/a_Edit', [('Var_Edit') : Var_Edit]))

